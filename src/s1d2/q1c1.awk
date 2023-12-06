function evaluate(ball_str, pos) {
	split(ball_str, sstr, ",")
	for (key in sstr) {
		split(sstr[key], ssstr, " ")
		#print sstr[key]
		if (ssstr[2]=="red") {
			if (ssstr[1]>12) {
				pos=1
				printf "red too large\n"
			}
		}
		if (ssstr[2]=="green") {
			if (ssstr[1]>13) {
				pos=1
				printf "green too large\n"
			}
		}
		if (ssstr[2]=="blue") {
			if (ssstr[1]>14) {
				pos=1
				printf "blue too large\n"
			}
		}
	}

	return pos
}

{
	print $0
	split($0, part, ":")
	split(part[1], spart1, " ")
	print spart1[2]
	split(part[2], spart2, ";")
	epos=0
	for (k in spart2) {
		print spart2[k]
		epos=evaluate(spart2[k], epos)
	}
	if (epos==0) {
		game_sum+=spart1[2]
		printf ">> added\n"
	} else {
		printf ">> invalid\n"
	}
	print game_sum
}

END{
	print game_sum
	printf "finished\n"
}
