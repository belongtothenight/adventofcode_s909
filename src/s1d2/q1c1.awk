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
	# game num
	# print substr($2, 1, length($2)-1)

	# entire line
	print $0
	
	# split string with ';'
	split($0, batch, ";")
	# print batch[1] # first bath mixed with game num
	split(batch[1], subbat, ":")
	# print subbat[1]
	split(subbat[1], ssubbat, " ")
	# print ssubbat[1]
	
	
	# print each color
	#print ssubbat[2] # game num
	print subbat[2] # first batch
	print batch[2] # second batch
	print batch[3] # third batch

	epos=0
	epos=evaluate(subbat[2], epos)
	epos=evaluate(batch[2], epos)
	epos=evaluate(batch[3], epos)

	if (epos==0) {
		game_sum+=ssubbat[2]
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
