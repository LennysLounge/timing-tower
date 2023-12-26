use unified_sim_model::Adapter;

use super::EditorCommand;

pub struct AdapterCommand {
    pub command: unified_sim_model::AdapterCommand,
}
impl AdapterCommand {
    pub fn execute(self, adapter: &mut Adapter) {
        adapter.send(self.command);
    }
}

impl From<AdapterCommand> for EditorCommand {
    fn from(value: AdapterCommand) -> Self {
        Self::AdapterCommand(value)
    }
}
