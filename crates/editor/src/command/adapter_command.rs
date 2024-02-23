use unified_sim_model::Adapter;

use super::EditorCommand;

pub struct AdapterCommand {
    pub command: unified_sim_model::AdapterCommand,
}
impl AdapterCommand {
    pub fn execute(self, adapter: &mut Option<&mut Adapter>) {
        if let Some(adapter) = adapter {
            adapter.send(self.command);
        }
    }
}

impl From<AdapterCommand> for EditorCommand {
    fn from(value: AdapterCommand) -> Self {
        Self::AdapterCommand(value)
    }
}
