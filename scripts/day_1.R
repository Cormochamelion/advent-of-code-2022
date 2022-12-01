cals <- readLines("../data/day_1.txt")

sum <- 0
sum_vec <- c()

for (cal in cals) {
	cal <- as.numeric(cal)
	if (is.na(cal)) {
		sum_vec <- c(sum_vec, sum)
		sum <- 0
	} else {
		sum <- sum + as.numeric(cal)
	}
}

sum_vec <- sort(sum_vec, decreasing = TRUE)

print(c("Best sum:", sum_vec[1]))
print(c("Sum top three:", sum(sum_vec[1:3])))
