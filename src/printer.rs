use similar::{TextDiff, ChangeTag};
use colored::Colorize;
use indoc::{indoc, formatdoc};

struct TestResult {
    name: String,
    testsuite: String,
    expected_stdout: String,
    actual_stdout: String,
    expected_stderr: String,
    actual_stderr: String,
    expected_status: i64,
    actual_status: i64
}

impl TestResult {
    pub fn from_name(name: &str, testsuite: &str) -> TestResult {
        TestResult {
            name: name.to_string(),
            testsuite: testsuite.to_string(),
            expected_stdout: "".to_string(),
            actual_stdout: "".to_string(),
            expected_stderr: "".to_string(),
            actual_stderr: "".to_string(),
            expected_status: 0,
            actual_status: 0
        }
    }
}

//pub fn run_test(testsuite: &TestFile, testcase: &TestCase) {
    //let result = TestResult{
        //name: testcase.name.clone(),
        //testsuite: testsuite.name.clone(),
        //stdout_diff: TextDiff::from_lines("", "").ops().to_vec(),
        //stderr_diff: TextDiff::from_lines("", "").ops().to_vec(),
        //expected_status: 0,
        //actual_status: 0
    //};
    //println!("{}", format_summary(&result, &compute_summary(&result)));
    //for line in compute_summary(&result) {
        //println!("{}", line);
    //}
//}
//

fn compute_summary(result: &TestResult) -> String {
    let mut summary = vec![];

    summary.push(compute_status(result.expected_status, result.actual_status));
    summary.extend(compute_diff("stdout", &result.expected_stdout, &result.actual_stdout));
    summary.extend(compute_diff("stderr", &result.expected_stderr, &result.actual_stderr));
    summary.join("")
}

fn compute_status(expected: i64, actual: i64) -> String {
    match expected == actual {
        true => String::new(),
        false => formatdoc!(r#"
            status code differs:
            expected: {}
            actual: {}
            "#,
            expected,
            actual),
    }
}

fn compute_diff(name: &str, expected: &str, actual: &str) -> Vec<String> {
    let mut diff_summary = vec![];
    let diff = TextDiff::from_lines(expected, actual);
    if !diff.ops().to_vec().is_empty() {
        diff_summary.push(formatdoc!(r#"
            {} differs:
            --- expected
            +++ actual
            "#, name));
    }
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        diff_summary.push(format!("{}{}", sign, change));
    }
    diff_summary
}



fn format_oneliner(result: &TestResult, success: bool) -> String {
    let success_msg = match success {
        true => "OK".green(),
        false => "FAILED".red(),
    };

    format!("test {}::{} ... {}",
        result.testsuite,
        result.name,
        success_msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_summary_successful() {
        // GIVEN
        let result = TestResult::from_name("mytest", "mysuite");

        // WHEN
        let summary = compute_summary(&result);

        // THEN
        assert_eq!("".to_string(), summary);
    }

    #[test]
    fn test_compute_summary_code_differs() {
        // GIVEN
        let mut result = TestResult::from_name("mytest", "mysuite");
        result.expected_status = 0;
        result.actual_status = 1;

        // WHEN
        let summary = compute_summary(&result);

        // THEN
        assert_eq!(indoc!{r#"
            status code differs:
            expected: 0
            actual: 1
            "#}, summary);
    }

    #[test]
    fn test_compute_summary_stdout_differs() {
        // GIVEN
        let mut result = TestResult::from_name("mytest", "mysuite");
        result.expected_stdout = "foo".to_string();
        result.actual_stdout = "fou".to_string();

        // WHEN
        let summary = compute_summary(&result);

        // THEN
        assert_eq!(indoc!{r#"
            stdout differs:
            --- expected
            +++ actual
            -foo
            +fou
            "#}, summary);
    }

    #[test]
    fn test_compute_summary_stderr_differs() {
        // GIVEN
        let mut result = TestResult::from_name("mytest", "mysuite");
        result.expected_stderr = "foo".to_string();
        result.actual_stderr = "fou".to_string();

        // WHEN
        let summary = compute_summary(&result);

        // THEN
        assert_eq!(indoc!{r#"
            stderr differs:
            --- expected
            +++ actual
            -foo
            +fou
            "#}, summary);
    }

    #[test]
    fn test_compute_summary_everything_differs() {
        // GIVEN
        let result = TestResult{
            name: "mytest".into(),
            testsuite: "mysuite".into(),
            expected_stdout: "foo".to_string(),
            actual_stdout: "fou".to_string(),
            expected_stderr: "bar".to_string(),
            actual_stderr: "baz".to_string(),
            expected_status: 0,
            actual_status: 1
        };

        // WHEN
        let summary = compute_summary(&result);

        // THEN
        assert_eq!(indoc!{r#"
            status code differs:
            expected: 0
            actual: 1
            stdout differs:
            --- expected
            +++ actual
            -foo
            +fou
            stderr differs:
            --- expected
            +++ actual
            -bar
            +baz
            "#}, summary);
    }

    #[test]
    fn test_format_oneliner_success() {
       // GIVEN
       let result = TestResult::from_name("mytest", "mysuite");
       let success = true;

       // WHEN
       let oneliner = format_oneliner(&result, success);

       // THEN
       assert_eq!(format!("test mysuite::mytest ... {}", "OK".green()), oneliner);
    }

    #[test]
    fn test_format_oneliner_failure() {
       // GIVEN
       let result = TestResult::from_name("supertest", "supersuite");
       let success = false;

       // WHEN
       let oneliner = format_oneliner(&result, success);

       // THEN
       assert_eq!(format!("test supersuite::supertest ... {}", "FAILED".red()), oneliner);
    }
}
