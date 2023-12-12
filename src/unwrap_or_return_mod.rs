pub enum ROO<T, E>{
    R(Result<T, E>),
    O(Option<T>),
}

pub trait UOR<T,E> {
    fn wrap(self) -> ROO<T,E>;
}

impl<T,E> UOR<T,E> for Result<T,E> {
    fn wrap(self) -> ROO<T,E>{
        ROO::R(self)
    }
}

impl<T> UOR<T,()> for Option<T> {
    fn wrap(self) -> ROO<T,()>{
        ROO::O(self)
    }
}


#[macro_export]
macro_rules! unwrap_or_return {
    ($lvl:expr) => {
        match qol::unwrap_or_return_mod::UOR::wrap($lvl) {
            qol::unwrap_or_return_mod::ROO::R(result) => {
                let Ok(thing) = result else {
                    return
                };
                thing
            },
            qol::unwrap_or_return_mod::ROO::O(option) => {
                let Some(thing) = option else {
                    return
                };
                thing
            }
        }
    };
   ($a:expr, $b:expr) => {
        match qol::unwrap_or_return_mod::UOR::wrap($a) {
            qol::unwrap_or_return_mod::ROO::R(result) => {
                let Ok(thing) = result else {
                    return $b
                };
                thing
            },
            qol::unwrap_or_return_mod::ROO::O(option) => {
                let Some(thing) = option else {
                    return $b
                };
                thing
            }
        }
    };
    ($a:expr , $($s:stmt);+ , $b:expr) => {
        match qol::unwrap_or_return_mod::UOR::wrap($a) {
            qol::unwrap_or_return_mod::ROO::R(result) => {
                let Ok(thing) = result else {
                    $($s)+
                    return $b
                };
                thing
            },
            qol::unwrap_or_return_mod::ROO::O(option) => {
                let Some(thing) = option else {
                    $($s)+
                    return $b
                };
                thing
            }
        }
    };
}