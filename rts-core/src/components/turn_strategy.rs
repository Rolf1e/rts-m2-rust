use pyo3::prelude::*;
use pyo3::types::{PyDict,PyString};
use pyo3::exceptions::PyValueError;

use crate::entity::game_actions::Action;
use crate::entity::player::TurnStrategyRequester;
use crate::exceptions::RtsException;
use crate::hooks::*;

pub enum TurnStrategy {
    AI(String),
    Dummy,
}

impl FromPyObject<'_> for Action {
    fn extract(ob: &'_ PyAny) -> PyResult<Self> {
        let dict = ob.downcast::<PyDict>().unwrap();
        let action_type = dict.get_item("type").unwrap().downcast::<PyString>().unwrap().to_str().unwrap();
        match action_type {
            "end_game" => Ok(Action::EndGame),
            "give_money_batch" => Ok(Action::GiveMoneyBatch),
            &_ => Err(PyValueError::new_err("Unknown action type")),
        }
    }
}

impl TurnStrategyRequester for TurnStrategy {
    fn request(&self) -> Result<Vec<Action>, RtsException> {
        match &self {
            TurnStrategy::AI(code) => {
                Python::with_gil(|py| -> Result<Vec<Action>, RtsException> {
                    let hash = calculate_hash(&code);
                    let module = match PyModule::from_code(py, code, &format!("{}.py", hash), &format!("{}", hash)) {
                        Ok(module) => module,
                        Err(err) => {
                            return Err(RtsException::PythonCompileCodeException(format!("Couldn't compile {}: {}", hash.to_string(), err)));
                        }
                    };
                    let result = module.call_method("ai", (), None).unwrap();
                    println!("Result of ai(): {}", result.repr().unwrap());
                    //TODO call python module to get actions
                    Ok(result.extract().unwrap())
                })
            },
            TurnStrategy::Dummy => {
                Ok(vec![])
            }
        }
    }
}

#[cfg(test)]
mod test_python {
    use super::TurnStrategy;
    use crate::entity::game_actions::Action;
    use crate::entity::player::TurnStrategyRequester;

    #[test]
    pub fn should_give_money_batch() {
        let turn_strategy = TurnStrategy::AI(
r#"
def ai():
    return [{"type": "give_money_batch"}]
"#.to_string()
        );

        match turn_strategy.request() {
            Ok(actions) => assert_eq!(actions, vec![Action::GiveMoneyBatch]),
            Err(_) => assert!(false)
        }
    }
}
