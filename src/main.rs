use swayipc::{Connection, Fallible};
use std::process::Command; 

struct Manager {
    connection: Connection,
}

impl Manager {
    fn new() -> Fallible<Self> {
        Ok(Self {
            connection: Connection::new()?
        })
    }

    fn rename(&mut self) -> Fallible<()> {
        let workspaces = self.connection.get_workspaces()?; 
        let current = workspaces.iter().find(|ws| ws.focused).ok_or("no focused workspace").unwrap();
        let idx = current.name.split("::").next()
            .unwrap() // should be impossible to not have a number
            .trim(); 
        let prompted = Command::new("rofi")
            .args(["-dmenu", "-p", "rename to: "])
            .output()?;
        
        let new_name = format!("{}:{}", idx, String::from_utf8_lossy(&prompted.stdout)
            .trim()
            .to_string()); 

        if !new_name.is_empty() {
            let cmd = format!("rename workspace \"{}\" to \"{}\"", current.name, new_name); 
            self.connection.run_command(&cmd)?;
        }
        Ok(())
    }
}

fn main() -> Fallible<()> {
    let mut manager = Manager::new()?;
    manager.rename()?; 

    Ok(())
}
