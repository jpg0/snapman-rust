use log::debug;
//use log::error;
//use log::info;
use log::warn;
use std::process::Command;
use std::str;
use std::path::PathBuf;
use std::convert::AsRef;
use strum_macros::{AsRefStr, EnumIter, EnumString};
use strum::IntoEnumIterator;
use std::str::FromStr;
use std::collections::HashMap;
use std::vec::IntoIter;


pub struct SnapraidCommand {
    pub binary: PathBuf,
    pub conf: PathBuf
}

#[derive(AsRefStr, Debug)]
enum SnapraidOp {
    diff,
    sync
}

#[derive(AsRefStr, EnumString, EnumIter, Debug, Eq, Hash, PartialEq)]
enum DiffOp {
    update,
    add,
    remove,
}

impl SnapraidCommand {
    fn default_params(&self) ->  IntoIter<&str> {
        return vec!["-c", self.conf.to_str().unwrap()].into_iter();
    }

    fn run(&self, op: SnapraidOp, params: IntoIter<&str>) -> Result<(String, i32), &str>{
        let args = self.default_params()
            .chain(vec![op.as_ref()].into_iter())
            .chain(params);

        debug!("Running {}", args.clone().fold(String::new(), |a, b| a + b + "\n"));

        let output = Command::new(self.binary.to_str().unwrap())
            .args(args)
            .output() //todo: timeout
            .expect("failed to execute process");

        if output.status.code() == Some(1) {
            warn!("Snapraid command returned non-zero (1) exit code!");
            warn!("{}", str::from_utf8(&output.stderr).unwrap());

            return Err("Snapraid command failed");
        }

        let stdout = str::from_utf8(&output.stdout).unwrap().to_owned();

        return Ok((stdout, output.status.code().unwrap()));
    }

    pub fn diff(&self) -> Result<DiffResult, String> {
        let (stdout, status) = self.run(SnapraidOp::diff, vec![].into_iter())?;
        return parse_diff(stdout, status);
    }

}


pub struct DiffResult {
    op_to_count: HashMap<DiffOp, i16>
}

impl DiffResult {
    pub fn added(&self) -> i16 { *self.op_to_count.get(&DiffOp::add).unwrap() }
}

pub fn parse_diff(stdout: String, _: i32) -> Result<DiffResult, String> {
    let mut map = HashMap::new();

    for op in DiffOp::iter() {
        map.insert(op, 0);
    }

    for line in stdout.split("\n") {
        debug!("> {}", line);

        if line.is_empty() {
            break;
        } else if line.starts_with("Loading state from") || line.starts_with("Comparing...") {
            continue;
        }

        let op_and_path: Vec<&str> = line.splitn(2, " ").collect();
        let opS = DiffOp::from_str(op_and_path[0]);

        if opS.is_err() {
           return Err(format!("Unknown operation type: {}", op_and_path[0]))
        }


        let op = opS.unwrap();
        let updated = map.get(&op).unwrap() + 1;
        map.insert(op, updated);
    }

    return Ok(DiffResult {
        op_to_count: map
    })
}
