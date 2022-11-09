use crate::snapraid_command::parse_diff;

#[cfg(test)]

    #[test]
    fn test_parse_diff_output() {
        let diff = parse_diff("Loading state from /var/snapraid.content...
Comparing...
update backup/timemachine/backup/jpg-mac-2021.sparsebundle/bands/1468
add backup/timemachine/backup/jpg-mac-2021.sparsebundle/mapped/56bd
remove backup/borgbackup/lbhome/root/lock.exclusive/lbv@210554223768372.778393-0
remove backup/borgbackup/lbhome/root/lock.roster

  138610 equal
      33 added
       2 removed
    1821 updated
       0 moved
       0 copied
       0 restored
There are differences!".to_string(), 0);

    match diff {
        Ok(val) => { assert_eq!(val.added(), 1) }
        Err(e) => { panic!("Failed to parse: {}", e) }
    }

    }
