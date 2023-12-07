function get_min_cnt(str, color) {
	min_cnt=0
	#print str
	#print color
	split(str, sstr, ",")
	for (i in sstr) {
		#print sstr[i]
		split(sstr[i], ssstr, " ")
		if (ssstr[2]==color) {
			if (ssstr[1]>min_cnt) {
				min_cnt=ssstr[1]
			}
		}
	}
	return min_cnt
}

{
	print $0

	# Split game count and ball count with ':'
	split($0, part, ":")
	#print part[2]

	# Replace ball count ';' with ','
	gsub(/;/, ",", part[2])
	#print part[2]

	# Retrieve minimum color num
	min_r=get_min_cnt(part[2], "red")
	min_g=get_min_cnt(part[2], "green")
	min_b=get_min_cnt(part[2], "blue")
	print min_r
	print min_g
	print min_b
	power=min_r*min_g*min_b
	print power
	power_sum+=power
}

END {
	printf "finished\n"
	print power_sum
}
