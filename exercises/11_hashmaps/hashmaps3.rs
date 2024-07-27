// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

// 更新球队信息的函数
fn update_team_scores(scores: &mut HashMap<String, Team>, team_name: &str, goals_scored: u8, goals_conceded: u8) {
    let entry = scores.entry(team_name.to_string()).or_insert(Team {
        goals_scored: 0,
        goals_conceded: 0,
    });
    entry.goals_scored += goals_scored;
    entry.goals_conceded += goals_conceded;
}

fn build_scores_table(results: &str) -> HashMap<String, Team> {
    // 使用球队名作为键，结构体作为值
    let mut scores = HashMap::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // 使用 unwrap 是为了简化代码，实际应用中应该处理错误
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // 更新 team_1 和 team_2 的进球和失球信息
        update_team_scores(&mut scores, team_1_name, team_1_score, team_2_score);
        update_team_scores(&mut scores, team_2_name, team_2_score, team_1_score);
    }

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
