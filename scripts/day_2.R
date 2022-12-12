library("purrr")
library("dplyr")
library("stringr")

recode_vec_play <- c("A" = "X", "B" = "Y", "C" = "Z")
recode_vec_score <- c("X" = 1, "Y" = 2, "Z" = 3)

vec <- c("Y", "X", "Z")


rps_outcome <- function(p1, p2, pos_vec) {
	if (p1 == p2) {
		return(3)
	}

	p2_pos <- which(pos_vec == p2)

	if (p2_pos == 1) {
		pos_vec <- pos_vec[c(3, 1, 2)]
	} else if (p2_pos == 3) {
		pos_vec <- pos_vec[c(2, 3, 1)]
	}

	p1_pos <- which(pos_vec == p1)

	if (p1_pos > p2_pos) {
		return(0)
	} else {
		return(6)
	}
}

strat <- readLines("../data/day_02.txt") %>%
	{data.frame(source = .)} %>%
       	mutate(opponent = str_extract(source, "^[ABC]"), play = str_extract(source, "[XYZ]$")) %>%
	mutate(opponent = recode(opponent, !!! recode_vec_play)) %>%
       	mutate(choice_score = as.numeric(recode(play, !!! recode_vec_score)))

outcome <- apply(strat, 1, function(strat_vec, pos_vec) {
	rps_score <- rps_outcome(strat_vec["opponent"], strat_vec["play"], pos_vec)
	tot_score <- rps_score + as.numeric(strat_vec["choice_score"])
	return(tot_score)
}, vec) %>% unlist()

cat("Total strat score:", sum(outcome))
