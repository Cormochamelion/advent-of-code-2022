cals <- readLines("../data/day_1.txt")

best_sum <- 0
sum <- 0

for (cal in cals) {
	cal <- as.numeric(cal)
	if (is.na(cal)) {
		if(sum > best_sum) {
			best_sum <- sum
		}
		sum <- 0
	} else {
		sum <- sum + as.numeric(cal)
	}
}

print(best_sum)
