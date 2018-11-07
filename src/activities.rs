// mod my_activity {
//     #[derive(Debug)]
//     pub struct Activity<'a> {
//         pub user_name: &'a str,
//         pub message: &'a str,
//         pub date: &'a str,
//         //I think a lifetime is needed because I access and assign the fields.
//     }
//     pub struct Activities {
//         pub activities: Vec<Activity>,
//     }

//     impl Activities {
//         pub fn add(&self, &mut activity: Activity) {
//             self.activities.push(activity)
//         }
//     }
// }
// #[cfg(test)]
// mod moretests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

#[derive(Debug)]
pub struct Activity<'a> {
    //&str needs a lifetime that trickles everywhere
    pub user_name: &'a str,
    pub message: &'a str,
    pub date: &'a str,
}
#[derive(Debug)]
pub struct Activities<'a> {
    pub activities: Vec<Activity<'a>>,
}

impl<'a> Activities<'a> {
    pub fn push(&mut self, activity: Activity<'a>) {
        self.activities.push(activity)
    }
    pub fn len(&self) -> usize {
        self.activities.len()
    }
    pub fn next(&self) -> &Activity {
        self.activities.iter().next().unwrap()
    }
    fn position(&self, username: &str) -> Option<usize> {
        self.activities
            .iter()
            .position(|ref mut x| x.user_name == username)
    }
    pub fn one_after(&self, username: &str) -> Option<&Activity> {
        let index = self.position(username);
        match index {
            Some(_) => Some(&self.activities[index.unwrap() - 1]),
            None => None,
        }
    }
    fn get(&self, username: &str) -> Option<&Activity> {
        let index = self.position(username);
        match index {
            Some(_) => Some(&self.activities[index.unwrap()]),
            None => None,
        }
    }
    pub fn get_user_reaction_time(&self, username: &str) -> Option<i64> {
        let one_after = self.one_after(username);
        let bot = self.get(username);
        match (one_after, bot) {
            (Some(_), Some(_)) => Some(
                one_after.unwrap().date.parse::<i64>().ok().unwrap()
                    - bot.unwrap().date.parse::<i64>().ok().unwrap(),
            ),
            (None, Some(_)) => None,
            (Some(_), None) => None,
            (None, None) => None,
        }
    }
}
