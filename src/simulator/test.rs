#![cfg(test)]

use std::str::FromStr;

use super::*;

#[test]
fn parse_parts_of_speech() {
    let tests = [
        (
            "#####\n#   #\n# # #\n#   #\n#####",
            PartOfSpeech::ParticleCollate,
        ),
        (
            "#####\n  # #\n#   #\n# #  \n#####",
            PartOfSpeech::Noun {
                islands: 2,
                depth: 0,
            },
        ),
        (
            " ####\n##  #\n# # #\n#  ##\n#### ",
            PartOfSpeech::Noun {
                islands: 2,
                depth: 1,
            },
        ),
        (
            "# ###\n#    \n# ###\n#   #\n#####",
            PartOfSpeech::Verb {
                islands: 2,
                depth: 0,
            },
        ),
        (
            "#####\n#   #\n# # #\n    #\n#####",
            PartOfSpeech::Verb {
                islands: 2,
                depth: 1,
            },
        ),
        (
            "## ##\n#   #\n#####\n#   #\n## ##",
            PartOfSpeech::Noun {
                islands: 1,
                depth: 0,
            },
        ),
    ];
    for (idx, (test, expect)) in tests.iter().enumerate() {
        let res = Symbol::from_str(test).unwrap().part_of_speech;
        assert_eq!(res, *expect, "testing idx {}: \n{}", idx, test);
    }
}
