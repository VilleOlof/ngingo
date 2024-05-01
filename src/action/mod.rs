pub mod add_cert;
pub mod create;
pub mod disable;
pub mod enable;
pub mod remove;
pub mod remove_cert;

pub enum Action {
    Create,
    AddCert,
    RemoveCert,
    Disable,
    Enable,
    Remove,
}

impl From<usize> for Action {
    fn from(index: usize) -> Self {
        match index {
            0 => Action::Create,
            1 => Action::Remove,
            2 => Action::Enable,
            3 => Action::Disable,
            4 => Action::AddCert,
            5 => Action::RemoveCert,
            _ => panic!("Invalid action"),
        }
    }
}

impl Action {
    pub fn run(&self) {
        match self {
            Action::Create => create::run(),
            Action::Remove => remove::run(),
            Action::Enable => enable::run(),
            Action::Disable => disable::run(),
            Action::AddCert => add_cert::run(),
            Action::RemoveCert => remove_cert::run(),
        }
    }
}
