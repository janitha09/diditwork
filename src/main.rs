fn main() {
    println!("Hello, world!");
}

mod my_module {
    use std::collections::HashMap;
    pub fn was_there_activity_after_the_warning(activity: &HashMap<&str, &str>) -> bool {
        match (activity.get("UserName"), activity.get("Message")) {
            (Some(&"UserName1"), Some(&"Message1")) => false, //no activity mine was the last
            _ => true,
        }
    }

    //I think the liftime is that same as the variable that recieves the value.
    pub fn get_bot_activity<'a>() -> HashMap<&'a str, &'a str> {
        let mut most_recent = HashMap::new();
        most_recent.insert("UserName", "UserName1");
        most_recent.insert("Message", "Message1");
        most_recent.insert("Date", "1533743653838");
        most_recent
    }
    pub fn get_immediate_activity_after_bot<'a>() -> HashMap<&'a str, &'a str> {
        let mut most_recent = HashMap::new();
        most_recent.insert("UserName", "UserName2");
        most_recent.insert("Message", "Message2");
        most_recent.insert("Date", "1533743653838");
        most_recent
    }
}

mod my_activity {
    #[derive(Debug)]
    pub struct Activity<'a> {
        pub user_name: &'a str,
        pub message: &'a str,
        pub date: &'a str,
        //I think a lifetime is needed because I access and assign the fields.
    }
}

//struct Activity
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }
    #[test]
    fn are_tuples_the_same() {
        let most_recent = (1i64, 3i64);
        let penultimate: (i64, i64) = (1, 3);
        assert_eq!(most_recent, penultimate);
    }
    #[test]
    fn was_the_last_activity_from_the_bot() {
        //do not care about this because I gaurantee that I will not
        //write again if I was the last person that wrote on to this
        //still useful to check for errors?
        //use unix time
        // https://users.rust-lang.org/t/problem-using-a-hashmap-with-option-type-for-value-component/5058
        use std::collections::HashMap;

        let mut most_recent = HashMap::new();
        most_recent.insert("UserName", "UserName1");
        most_recent.insert("Message", "Message1");
        most_recent.insert("Date", "1541191586");

        // let mut penultimate = HashMap::new();
        // penultimate.insert("UserName", "UserName1");
        // penultimate.insert("Message", "Message1");
        // penultimate.insert("Date", "1541191586");
        // assert_eq!(most_recent, penultimate);
        assert_eq!(most_recent.get(&"UserName"), Some(&"UserName1"));
        assert_eq!(most_recent.get(&"Message"), Some(&"Message1"));
    }
    #[test]
    fn check_calls_in_hashmap() {
        use std::collections::HashMap;
        let mut contacts = HashMap::new();

        contacts.insert("Daniel", "798-1364");
        contacts.insert("Ashley", "645-7689");
        contacts.insert("Katie", "435-8291");
        contacts.insert("Robert", "956-1745");

        // Takes a reference and returns Option<&V>
        match contacts.get(&"Daniel") {
            Some(&number) => println!("Calling Daniel: {}", call(number)),
            _ => println!("Don't have Daniel's number."),
        }
        assert_eq!(contacts.get(&"Daniel"), Some(&"798-1364"));
    }
    fn call(number: &str) -> &str {
        match number {
            "798-1364" => {
                "We're sorry, the call cannot be completed as dialed.
            Please hang up and try again."
            }
            "645-7689" => {
                "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?"
            }
            _ => "Hi! Who is this again?",
        }
    }
    #[test]
    fn is_it_more_than_x_days_after_the_warning() {
        //maybe use a struct like the viking example
        // https://doc.rust-lang.org/std/collections/struct.HashMap.html
        use std::collections::HashMap;

        let mut most_recent = HashMap::new();
        most_recent.insert("UserName", "UserName1");
        most_recent.insert("Message", "Message1");
        most_recent.insert("Date", "1533743653838");

        use my_module::*;
        assert_eq!(was_there_activity_after_the_warning(&most_recent), false);
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        println!("epoch seconds {:?}", since_the_epoch);
        let in_ms = since_the_epoch.as_secs() as i64 * 1000
            + since_the_epoch.subsec_nanos() as i64 / 1000000;
        println!("epoch milliseconds {:?}", in_ms);

        let my_string = "27".to_string(); // `parse()` works with `&str` and `String`!
        let my_int = my_string.parse::<i64>().unwrap();

        println!("int {:?} {:?}", my_string, my_int);

        assert_eq!(most_recent.get(&"Date"), Some(&"1533743653838"));
        let activity_date = most_recent.get(&"Date").unwrap().to_string();
        let activity_date_int = activity_date.parse::<i64>().unwrap();
        println!("activity date {:?} {:?}", activity_date, activity_date_int);
        let time_elapsed = (in_ms - activity_date_int) / 86400 / 1000;
        println!("time elapsed {:?} days", time_elapsed)

        // use chrono::offset::{TimeZone, Utc};
        // let utc = Utc;
        // let d1 = Utc::now();
        // let d2 = utc.datetime_from_str(&"Jan 30 02:19:17 2018", "%b %d %H:%M:%S %Y")
        //     .unwrap();
        // let duration = d2.signed_duration_since(d1);
        // println!("Duration: {:?}", duration);
        // println!("As whole days: {:?}", duration.num_days());
    }

    #[test]
    fn test_was_there_activity_after_the_warning() {
        use std::collections::HashMap;

        let mut most_recent = HashMap::new();
        most_recent.insert("UserName", "UserName1");
        most_recent.insert("Message", "Message1");
        most_recent.insert("Date", "1533743653838");

        use my_module::*;
        assert_eq!(was_there_activity_after_the_warning(&most_recent), false);
        //what is the differene in time between the bot activity and the immediate next activity
        //time between two activities.
        //find the bots activity
    }
    #[test]
    fn if_there_was_activity_how_long_did_it_take() {
        use my_module::*;
        let bot_activity = get_bot_activity();
        let bot_activity_date = bot_activity.get("Date").unwrap().parse::<i64>().unwrap();
        assert_eq!(bot_activity_date, 1533743653838);
        assert_eq!(bot_activity.get("UserName"), Some(&"UserName1"));
        let immediate_activity_after_bot = get_immediate_activity_after_bot();
        let immediate_activity_after_bot_date = immediate_activity_after_bot
            .get("Date")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        assert_eq!(immediate_activity_after_bot_date, 1533743653838);
        assert_eq!(
            immediate_activity_after_bot.get("UserName"),
            Some(&"UserName2")
        );
        assert_eq!(immediate_activity_after_bot_date - bot_activity_date, 0)
    }
    // https://docs.rs/elastic_requests/0.20.10/elastic_requests/
    #[test]
    fn find_an_activity_in_a_vector_of_activities() {
        use my_activity::Activity;
        let activity1 = Activity {
            user_name: "UserName1",
            message: "Message1",
            date: "1533743653838",
        };
        assert_eq!(activity1.user_name, "UserName1");
        let mut activities = Vec::new();
        activities.push(activity1);
        assert_eq!(activities.len(), 1);

        assert_eq!("UserName1", activities.iter().next().unwrap().user_name);
        // https://github.com/rust-lang/rust-by-example/issues/390#issuecomment-69660693

        assert_eq!(
            "UserName1",
            activities
                .iter()
                .find(|x| (**x).user_name == "UserName1") //this is equivalent to .find(|ref mut x| x.user_name == "UserName1")
                .unwrap()
                .user_name
        );
        let activity2 = Activity {
            user_name: "UserName2",
            message: "Message2",
            date: "1533743653838",
        };
        activities.push(activity2);

        assert_eq!(activities.len(), 2);
        assert_eq!("UserName1", activities.iter().next().unwrap().user_name);

        assert_eq!(
            "UserName2",
            activities
                .into_iter()
                .find(|x| x.user_name == "UserName2") //this is equivalent to .find(|ref mut x| x.user_name == "UserName1")
                .unwrap()
                .user_name
        );
    }
}
