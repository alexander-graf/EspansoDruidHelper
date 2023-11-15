use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use druid::widget::{Button, Flex, TextBox, CrossAxisAlignment, SizedBox};
use std::process::Command;
use std::path::Path;
use dirs;
use std::fs::OpenOptions;
use std::io::Write;



#[derive(Clone, Data, Lens, Debug)]
struct MyApp {
    trigger: String,
    replace: String,
}

#[cfg(unix)]
fn open_folder() -> std::io::Result<()> {
    let mut path = dirs::home_dir().ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found"))?;
    path.push(".config/espanso/match");
    Command::new("xdg-open").arg(path).status()?;
    Ok(())
}

#[cfg(windows)]
fn open_folder() -> std::io::Result<()> {
    let mut path = dirs::home_dir().ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found"))?;
    path.push(".config/espanso/match");
    Command::new("cmd").args(["/C", "start", path.to_str().unwrap()]).status()?;
    Ok(())
}

impl MyApp {
  fn append_to_file(&self) -> std::io::Result<()> {
    let mut path = dirs::home_dir().ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found"))?;
    path.push(".config/espanso/match/kustom.yml");
      
    let filename = path.to_str().unwrap();

    let file_exists = Path::new(filename).exists();
    let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(filename)?;

    if !file_exists {
      file.write_all(b"matches:\n")?;
      println!("Created new file.");
    }
    else {
      println!("File already exists.");
    }

    let formatted_replace = if self.replace.contains('\n') {
        let indented_replace = self.replace
          .lines()
          .map(|line| format!("   {}", line))  // Add as many spaces as you need here
          .collect::<Vec<_>>()
          .join("\n");
        format!("\n- trigger: '{}'\n  replace: |\n{}\n", self.trigger, indented_replace)
      } else {
        format!("\n- trigger: '{}'\n  replace: '{}'\n", self.trigger, self.replace)
      };
  
      file.write_all(formatted_replace.as_bytes())?;
      Ok(())
    }
  // other methods...
}

fn main() {
    run().unwrap();
}

fn run() -> std::io::Result<()> {
    let main_window = WindowDesc::new(ui_builder())
        .title("Espanso Helper Druid")
        .window_size((420.0, 240.0));

    let data = MyApp {
        trigger: "".to_owned(),
        replace: "".to_owned(),
    };

    let mut app = MyApp {
        trigger: "hello".to_string(),
        replace: "world".to_string(),
        // initialize other fields...
    };
    

    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Failed to launch application");
    Ok(())
}

fn ui_builder() -> impl Widget<MyApp> {
    let trigger_textbox = TextBox::new()
        .with_placeholder("Trigger")
        .lens(MyApp::trigger);

        let replace_textbox = SizedBox::new(
            TextBox::multiline()
                .with_placeholder("Replace")
                .lens(MyApp::replace)
        ).height(100.0);
        

    let open_folder_button = Button::new("Open Folder")
        .on_click(|_ctx, _data: &mut MyApp, _env| {
            open_folder().expect("Failed to open folder");
        });

  let append_button = Button::new("Append")
      .on_click(|_ctx, data: &mut MyApp, _env| {
          if data.trigger.is_empty() {
              println!("Trigger is empty. Operation cancelled.");
          } else {
              data.append_to_file().expect("Failed to append to file");
          }
      });

    let trigger_and_button = Flex::row()
        .with_child(trigger_textbox)
        .with_child(append_button);

    let replace_and_open_folder = Flex::row()
        .with_child(replace_textbox)
        .with_child(open_folder_button);

        Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start) // Add this line for left alignment
        .with_child(trigger_and_button)
        .with_child(replace_and_open_folder)
}