use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::ast::visitor::expr::ExprVisitor;
use crate::ast::visitor::stmt::StmtVisitor;
use crate::callable::instance::QalamInstance;
use crate::callable::QalamCallable;
use crate::callable::function::QalamFunction;
use crate::callable::class::QalamClass;
use crate::token::{ Token, TokenType };
use crate::literal::{Literal, QalamArray};
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::callable::global::{ ClockFn, PowFn, MaxFn, MinFn, LenFn, NumFn, StrFn, TypeofFn, SubstrFn, IndexOfFn, ReplaceFn, RandomFn, RandomIntFn };
use crate::callable::global::is_usize;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::hashable::{HashableMap, HashableRcRefCell};



pub struct Interpreter {
  pub globals: Rc<RefCell<Environment>>,
  pub environment: Rc<RefCell<Environment>>,
  pub locals: HashMap<Expr, usize>
}

impl Interpreter {
  pub fn init() -> Self {
    let globals = Rc::new(RefCell::new(Environment::init(None)));
    Self::add_global(globals.clone(), "clock", ClockFn::init());
    Self::add_global(globals.clone(), "pow", PowFn::init());
    Self::add_global(globals.clone(), "max", MaxFn::init());
    Self::add_global(globals.clone(), "min", MinFn::init());
    Self::add_global(globals.clone(), "len", LenFn::init());
    Self::add_global(globals.clone(), "str2num", NumFn::init());
    Self::add_global(globals.clone(), "str", StrFn::init());
    Self::add_global(globals.clone(), "typeof", TypeofFn::init());
    Self::add_global(globals.clone(), "substr", SubstrFn::init());
    Self::add_global(globals.clone(), "index_of", IndexOfFn::init());
    Self::add_global(globals.clone(), "replace", ReplaceFn::init());
    Self::add_global(globals.clone(), "random", RandomFn::init());
    Self::add_global(globals.clone(), "random_int", RandomIntFn::init());
    return Self {
      globals: globals.clone(),
      environment: globals.clone(),
      locals: HashMap::new()
    }
  }

  pub fn resolve(&mut self, expr: &Expr, depth: usize) -> Result<(), RuntimeError> {
    self.locals.insert(expr.clone(), depth);
    return Ok(())
  }

  fn add_global<F>(globals: Rc<RefCell<Environment>>, name: &str, func: F)
  where F: QalamCallable + 'static
   {
    globals.borrow_mut().define(name.to_string(), Some(Literal::Callable(Box::new(func))))
  }

  fn evaluate(&mut self, expr: &Expr) -> Result<Option<Literal>, RuntimeError> {
    expr.accept(self)
  }

  fn flip_bool(value: bool, flip: bool) -> bool {
    if flip {
      return !value;
    } else {
      return value;
    }
  }

  fn is_truthy(value: Option<Literal>, flip: bool) -> Option<Literal> {
    match value {
      Some(val) => {
        if let Literal::Bool(bool_val) = val {
          return Some(Literal::Bool(Self::flip_bool(bool_val, flip)))
        }
      },
      None => {
        return Some(Literal::Bool(Self::flip_bool(false, flip)))
      }
    }
    return Some(Literal::Bool(Self::flip_bool(true, flip)));
  }

  fn is_equal(&mut self, a: Option<Literal>, b: Option<Literal>, flip: bool) -> Option<Literal> {
    match a {
      Some(a_val) => {
        match b {
          Some(b_val) => {
            if let (Literal::Number(a_val), Literal::Number(b_val)) = (a_val.clone(), b_val.clone()) {
              return Some(Literal::Bool(Self::flip_bool(a_val == b_val, flip)));
            }

            if let (Literal::Bool(a_val), Literal::Bool(b_val)) = (a_val.clone(), b_val.clone()) {
              return Some(Literal::Bool(Self::flip_bool(a_val == b_val, flip)));
            }

            if let (Literal::String(a_val), Literal::String(b_val)) = (a_val, b_val) {
              return Some(Literal::Bool(Self::flip_bool(a_val == b_val, flip)));
            }

            return Some(Literal::Bool(Self::flip_bool(false, flip)))
          },
          None => {
            return Some(Literal::Bool(Self::flip_bool(false, flip)))
          }
        }
      },
      None => {
        match b {
          Some(_) => {
            return Some(Literal::Bool(Self::flip_bool(false, flip)))
          },
          None => {
            return Some(Literal::Bool(Self::flip_bool(true, flip)))
          }
        }
      }
    }
  }

  pub fn execute_block(&mut self, statements: &mut Vec<Stmt>, environment: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
    let previous = self.environment.clone();
    self.environment = environment.clone();
    // println!("execute block, env: {:?}", self.environment);
    for stmt in statements.iter_mut() {
      match self.execute(stmt) {
        Ok(_) => {},
        Err(e) => {
          self.environment = previous.clone();
          return Err(e)
        }
      }
    }

    self.environment = previous.clone();
    return Ok(());
  }

  pub fn interpret(&mut self, mut statements: Vec<Stmt>) -> Result<(), RuntimeError> {
    for stmt in statements.iter_mut() {
      self.execute(stmt)?;
    }
    Ok(())
  }

  fn execute(&mut self, stmt: &mut Stmt) -> Result<(), RuntimeError> {
    stmt.accept(self)
  }

  fn lookup_variable(&mut self, name: &Token, expr: &Expr) -> Result<Option<Literal>, RuntimeError> {
    let distance = self.locals.get(&expr);
    if let Some(distance) = distance {
      return Ok(Environment::get_at(self.environment.clone(), *distance, name.lexeme.to_string())?)
    } else {
      return Ok(self.globals.as_ref().borrow().get(name)?)
    }
  }

  fn evaluate_index(&mut self, index: &Box<Expr>, bracket: &Token) -> Result<usize, RuntimeError> {
    let index = self.evaluate(index)?;
      if let Some(index) = index.clone() {
        match index {
          Literal::Number(val) => {
            if !is_usize(*val) {
              return Err(RuntimeError::init(bracket, format!("index must be a positive integer!")))
            } else {
              return Ok(*val as usize);
            }
          },
          _ => {
            return Err(RuntimeError::init(bracket, format!("index must be a number!")))
          }
        };
      } else {
        return Err(RuntimeError::init(bracket, format!("index cannot not be ghaib!")))
      }
  }
}

impl ExprVisitor for Interpreter {
  type R = Result<Option<Literal>, RuntimeError>;
  fn visit_literal(&mut self, expr: &Option<Literal>) -> Self::R {
      return Ok(expr.clone())
  }

  fn visit_array(&mut self, values: &Vec<Expr>) -> Self::R {
    let mut qalam_array = QalamArray::init();
    for value in values.iter() {
      qalam_array.elements.push(self.evaluate(value)?);
    }
    return Ok(Some(Literal::Array(HashableRcRefCell::init(qalam_array))))
  }

  fn visit_grouping(&mut self, expression: &Box<Expr>) -> Self::R {
      return self.evaluate(expression)
  }

  fn visit_unary(&mut self, operator: &Token, right: &Box<Expr>) -> Self::R {
      let right_val = self.evaluate(right)?;
      match operator.token_type {
        TokenType::Minus => {
          if let Some(val) = right_val {
            match val {
              Literal::Number(num) => {
                return Ok(Some(Literal::Number(-num)))
              },
              _ => {
                return Err(RuntimeError::init(operator, String::from("Operand must be a number.")))
              }
            }
          } else {
            return Err(RuntimeError::init(operator, String::from("Operand must be a number.")))
          }
        },
        TokenType::Bang => {
          return Ok(Self::is_truthy(right_val, true))
        },
        _ => {}
      }
      return Ok(None); // idk about this??
  }

  fn visit_binary(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::R {
      let left_val = self.evaluate(left)?;
      let right_val = self.evaluate(right)?;

      if let (Some(left_val), Some(right_val)) = (left_val.clone(), right_val.clone()) {
        match operator.token_type {
          TokenType::Minus => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Number(left_val - right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::Slash => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Number(left_val / right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::Star => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Number(left_val * right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::Plus => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val.clone(), right_val.clone()) {
              return Ok(Some(Literal::Number(left_val + right_val)));
            }

            if let (Literal::String(left_val), Literal::String(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::String(left_val + right_val.as_str())));
            }

            return Err(RuntimeError::init(operator, String::from("Operands must be two numbers or two strings.")))
          },
          TokenType::Greater => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Bool(left_val > right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::GreaterEqual=> {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Bool(left_val >= right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::Less => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Bool(left_val < right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::LessEqual => {
            if let (Literal::Number(left_val), Literal::Number(right_val)) = (left_val, right_val) {
              return Ok(Some(Literal::Bool(left_val <= right_val)));
            } else {
              return Err(RuntimeError::init(operator, String::from("Operands must be numbers.")))
            }
          },
          TokenType::BangEqual => {
            return Ok(self.is_equal(Some(left_val), Some(right_val), true))
          },
          TokenType::EqualEqual => {
            return Ok(self.is_equal(Some(left_val), Some(right_val), false))
          }
          _ => {}
        }
      } else {
        // handle equality here for both null
        match operator.token_type {
          TokenType::EqualEqual => {
            return Ok(self.is_equal(left_val, right_val, false))
          },
          TokenType::BangEqual => {
            return Ok(self.is_equal(left_val, right_val, true))
          },
          _ => {
            return Err(RuntimeError::init(operator, String::from("Invalid operation. This should not happen!")))
          }
        }
      }
    
      return Ok(None); // idk about this??
  }

  

  fn visit_variable(&mut self, name: &Token) -> Self::R {
      return self.lookup_variable(name, &Expr::Variable { name: name.clone() });
  }

  fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> Self::R {
      let res_value = self.evaluate(value)?;
      let distance = self.locals.get(&Expr::Assign { name: name.clone(), value: value.clone() });
      if let Some(distance) = distance {
        Environment::assign_at(self.environment.clone(), *distance, name, res_value.clone())?;
      } else {
        self.globals.borrow_mut().assign(name, res_value.clone())?;
      }
      return Ok(res_value)
  }

  fn visit_logical(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::R {
      let left = self.evaluate(left)?;
      match operator.token_type {
        TokenType::Or => {
          match Self::is_truthy(left.clone(), false) {
            Some(val) => {
              match val {
                Literal::Bool(val) => {
                  if val {
                    return Ok(left)
                  }
                },
                _ => {
                  eprintln!("Something went wrong in Interpreter.visit_logical.");
                  // this should never happen, is_truthy always returns bool literal
                }
              }
            },
            None => {
              eprintln!("Something went wrong in Interpreter.visit_logical.");
              // this should never happen, is_truthy always returns bool literal
            }
          }
        },
        _ => {
          match Self::is_truthy(left.clone(), true) {
            Some(val) => {
              match val {
                Literal::Bool(val) => {
                  if val {
                    return Ok(left)
                  }
                },
                _ => {
                  eprintln!("Something went wrong in Interpreter.visit_logical.");
                  // this should never happen, is_truthy always returns bool literal
                }
              }
            },
            None => {
              eprintln!("Something went wrong in Interpreter.visit_logical.");
              // this should never happen, is_truthy always returns bool literal
            }
          }
        }
      };

      return self.evaluate(right);
  }

  fn visit_call(&mut self, callee: &Box<Expr>, paren: &Token, arguments: &Vec<Expr>) -> Self::R {
      let callee = self.evaluate(&callee)?;
      let mut args = Vec::new();
      for arg in arguments.iter() {
        args.push(self.evaluate(arg)?);
      }

      match callee {
        Some(literal) => {
          match literal {
            Literal::Callable(mut function) => {
              if args.len() != function.arity() {
                return Err(RuntimeError::init(paren, format!("Expected {} arguments but got {}.", function.arity(), args.len())))
              }
              return Ok(function.call(self, args, paren)?)
            },  
            _ => {
              return Err(RuntimeError::init(paren, String::from("Can only call functions and classes.")))
              // throw runtime error, can ONLY call callable
            }
          }
        },
        None => {
          return Err(RuntimeError::init(paren, String::from("Can only call functions and classes.")))
          // throw runtime error, cannot call null
        }
      }
  }

  fn visit_get(&mut self, object: &Box<Expr>, name: &Token) -> Self::R {
      let object = self.evaluate(object)?;
      if let Some(object) = object {
        if let Literal::Instance(object) = object {
          return Ok(QalamInstance::get(object.clone(), name)?)
          //return Ok(object.0.borrow().get(name)?);
        }
      }

      return Err(RuntimeError::init(name, format!("Only instances have properties.")))
  }

  fn visit_set(&mut self, object: &Box<Expr>, name: &Token, value: &Box<Expr>) -> Self::R {
      let object = self.evaluate(object)?;
      if let Some(object) = object {
        if let Literal::Instance(object) = object {
          let value = self.evaluate(value)?;
          object.0.borrow_mut().set(name, value.clone());
          return Ok(value);
        }
      }

      return Err(RuntimeError::init(name, format!("Only instances have fields.")))
  }

  fn visit_this(&mut self, keyword: &Token) -> Self::R {
      return self.lookup_variable(keyword, &Expr::This { keyword: keyword.clone() });
  }

  fn visit_super(&mut self, keyword: &Token, method: &Token) -> Self::R {
      let distance = self.locals.get(&Expr::Super { keyword: keyword.clone(), method: method.clone() }).unwrap();
      let superclass;
      if let Some(Literal::Callable(class)) = Environment::get_at(self.environment.clone(), *distance, String::from("ulya"))? {
        if let Some(class) = class.as_any().downcast_ref::<QalamClass>() {
          superclass = class.clone();
        } else {
          return Err(RuntimeError::init(keyword, format!("superclass does not exist!")));
        }
      } else {
        return Err(RuntimeError::init(keyword, format!("superclass does not exist!")));
      }
      let object;
      if let Some(Literal::Instance(instance)) = Environment::get_at(self.environment.clone(), distance - 1, String::from("nafs"))? {
        object = instance;
      } else {
        return Err(RuntimeError::init(keyword, format!("Cannot find instance!")));
      }

      let actual_method = superclass.find_method(&method.lexeme);
      if let Some(actual_method) = actual_method {
        if let Some(actual_method) = actual_method.as_any().downcast_ref::<QalamFunction>() {
          let bind = actual_method.bind(object);
          return Ok(Some(Literal::Callable(Box::new(bind))));
        } else {
          return Err(RuntimeError::init(method, format!("method is not a function!")))
        }
      } else {
        return Err(RuntimeError::init(method, format!("Undefined method '{}'.", method.lexeme)));
      }
  }

  

  fn visit_get_indexed(&mut self, object: &Box<Expr>, index: &Box<Expr>, bracket: &Token) -> Self::R {
      let object = self.evaluate(object)?;
      if let Some(object) = object {
        match object {
          Literal::Array(arr) => {
            // do something
            let idx = self.evaluate_index(index, bracket)?;
            if idx > arr.0.as_ref().borrow().elements.len() - 1 {
              return Err(RuntimeError::init(bracket, format!("index is out of range!")));
            }
            let val = &arr.0.as_ref().borrow().elements[idx];
            return Ok(val.clone());
          },
          Literal::String(str) => {
            // do something
            let idx = self.evaluate_index(index, bracket)?;
            if idx > str.len() - 1 {
              return Err(RuntimeError::init(bracket, format!("index is out of range!")))
            }
            let val = str.chars().nth(idx).unwrap();
            return Ok(Some(Literal::String(val.to_string())))
          },
          _ => {
            return Err(RuntimeError::init(bracket, format!("Can only index string and array!")))
          } 
        }
      } else {
        return Err(RuntimeError::init(bracket, format!("Cannot index ghaib!")))
      }
  }

  fn visit_set_indexed(&mut self, object: &Box<Expr>, index: &Box<Expr>, value: &Box<Expr>, bracket: &Token) -> Self::R {
      let object = self.evaluate(object)?;
      let value = self.evaluate(value)?;
      if let Some(Literal::Array(arr)) = object {
        let idx = self.evaluate_index(index, bracket)?;
        if idx > arr.0.as_ref().borrow().elements.len() - 1 {
          return Err(RuntimeError::init(bracket, format!("index is out of range!")))
        }
        arr.0.as_ref().borrow_mut().elements[idx] = value.clone();
        return Ok(value);
      } else {
        return Err(RuntimeError::init(bracket, format!("cannot access non-array-like by index!")))
      }
  }
}

impl StmtVisitor for Interpreter {
  type R = Result<(), RuntimeError>;
  fn visit_expression(&mut self, expression: &Expr) -> Self::R {
      match self.evaluate(expression) {
        Ok(_) => {
          return Ok(())
        },
        Err(e) => {
          return Err(e)
        }
      }
  }

  fn visit_print(&mut self, expression: &Expr) -> Self::R {
      let value = match self.evaluate(expression) {
        Ok(val) => val,
        Err(e) => {
          return Err(e)
        }
      };
      if let Some(val) = value {
        println!("{}", val.to_qalam_string());
      } else {
        println!("ghaib")
      }
      Ok(())
  }

  fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Self::R {
      let value = match initializer {
        Some(val) => {
          self.evaluate(val)?
        },
        None => None
      };
      self.environment.borrow_mut().define(name.lexeme.to_owned(), value);
      Ok(())
  }

  fn visit_block(&mut self, statements: &mut Vec<Stmt>) -> Self::R {
      self.execute_block(statements, Rc::new(RefCell::new(Environment::init(Some(self.environment.clone())))))?;
      return Ok(())
  }

  fn visit_if(&mut self, condition: &Expr, then: &mut Box<Stmt>, else_branch: &mut Option<Box<Stmt>>) -> Self::R {
      let eval_cond = self.evaluate(condition)?;
      match Self::is_truthy(eval_cond, false) {
        Some(val) => {
          match val {
            Literal::Bool(cond) => {
              if cond {
                self.execute( then)?;
              } else {
                match else_branch {
                  Some(else_stmt) => {
                    self.execute(else_stmt)?;
                  },
                  None => {
                    // do nothing
                  }
                }
              }
            },
            _ => {
              eprintln!("Something went wrong in Interpreter.visit_if")
              // this should never happen, is_truthy always returns a bool literal
            }
          }
        },
        None => {
          eprintln!("Something went wrong in Interpreter.visit_if")
          // this should never happen, is_truthy always returns a bool literal
        }
      }

      return Ok(())
  }

  fn visit_while(&mut self, condition: &Expr, body: &mut Box<Stmt>) -> Self::R {
      let mut iterate = match Self::is_truthy(self.evaluate(condition)?, false) {
        Some(val) => {
          match val {
            Literal::Bool(val) => val,
            _ => false
          }
        },
        None => false
      };
      while iterate {
        self.execute(body)?;
        iterate = match Self::is_truthy(self.evaluate(condition)?, false) {
          Some(val) => {
            match val {
              Literal::Bool(val) => val,
              _ => false
            }
          },
          None => false
        };
      }
      return Ok(())
  }

  fn visit_function(&mut self, name: &Token, params: &Vec<Token>, body: &mut Vec<Stmt>) -> Self::R {
      let function = QalamFunction::init(Stmt::Function { name: name.clone(), params: params.clone(), body: body.clone() },  self.environment.clone(), false);
      self.environment.borrow_mut().define(name.lexeme.to_string(), Some(Literal::Callable(Box::new(function))));
      return Ok(());
  }

  fn visit_return(&mut self, _keyword: &Token, value: &Option<Expr>) -> Self::R {
      let mut val = None;
      match value {
        Some(expr) => {
          val = self.evaluate(expr)?
        },
        None => {}
      };

      return Err(RuntimeError::init_return(val));
  }

  fn visit_class(&mut self, name: &Token, methods: &mut Vec<Stmt>, superclass: &Option<Expr>) -> Self::R {
      let mut option_superclass = None;
      if let Some(superclass) = superclass {
        if let Some(Literal::Callable(eval_superclass)) = self.evaluate(superclass)? {
          if let Some(eval_superclass) = eval_superclass.as_any().downcast_ref::<QalamClass>() {
            option_superclass = Some(Box::new(eval_superclass.clone()));
          } else {
            // evaluated superclass is not a class
            return Err(RuntimeError::init(name, "Superclass must be a class".to_string()))
          }
        } else {
          return Err(RuntimeError::init(name, "Superclass must be a class.".to_string()))
        }
      }

      self.environment.borrow_mut().define(name.lexeme.to_owned(), None);
      if let Some(_) = option_superclass.clone() {
        self.environment = Rc::new(RefCell::new(Environment::init(Some(self.environment.clone()))));
        self.environment.borrow_mut().define(String::from("ulya"), Some(Literal::Callable(option_superclass.clone().unwrap())))
      }
      let mut hash_methods: HashableMap<String, Box<dyn QalamCallable>> = HashableMap::new();
      for method in methods.iter() {
        if let Stmt::Function { name, params, body } = method {
          let func = QalamFunction::init(Stmt::Function { name: name.clone(), params: params.clone(), body: body.clone() }, self.environment.clone(), name.lexeme.eq(&String::from("khalaq")));
          hash_methods.insert(name.lexeme.to_owned(), Box::new(func));
        } else {
          return Err(RuntimeError::init(name, format!("method is not a function!")))
        }
      }
      let class = QalamClass::init(name.lexeme.to_owned(), hash_methods, option_superclass.clone());
      if option_superclass.is_some() {
        let enclosing = self.environment.as_ref().borrow().enclosing.as_ref().unwrap().clone();
        self.environment = enclosing;
      }
      self.environment.borrow_mut().assign(name, Some(Literal::Callable(Box::new(class))))?;
      
      return Ok(());
  }
}