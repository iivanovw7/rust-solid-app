use actix_web::web::Data;
use std::collections::HashMap;
use std::sync::Mutex;

pub type State<'a, T> = HashMap<&'a str, T>;
pub type AppState<'a, T> = Data<Mutex<State<'a, T>>>;

pub fn new_state<'a, T>() -> AppState<'a, T> {
    let state = State::<T>::new();
    Data::new(Mutex::new(state))
}

#[allow(dead_code)]
pub fn set<'a, T>(data: AppState<'a, T>, key: &'a str, value: T) -> Option<T> {
    let mut hashmap = data.lock().expect("Could not acquire lock");
    hashmap.insert(key, value)
}

#[allow(dead_code)]
pub fn get<'a, T>(data: AppState<'a, T>, key: &'a str) -> Option<T>
where
    T: Clone,
{
    let hashmap = data.lock().expect("Could not acquire lock");
    Some(hashmap.get(key)?.to_owned())
}

#[allow(dead_code)]
pub fn delete<'a, T>(data: AppState<'a, T>, key: &'a str) -> Option<T> {
    let mut hashmap = data.lock().expect("Could not acquire lock");
    hashmap.remove(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::helpers::tests::app_state;

    #[test]
    fn it_creates_new_application_state_and_sets_and_reads_it() {
        let data = app_state();
        set(data.clone(), "testing", "123".into());
        let value = get(data, "testing");
        assert_eq!(value, Some("123".to_string()));
    }

    #[test]
    fn it_removes_an_entry_in_application_state() {
        let data = app_state();
        set(data.clone(), "testing", "123".into());
        let value = get(data.clone(), "testing");
        assert_eq!(value, Some("123".to_string()));
        delete(data.clone(), "testing");
        let value = get(data, "testing");
        assert_eq!(value, None);
    }
}
