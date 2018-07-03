while (<>) {
  $h = $_; 
  $s = <>; 
  $seqs{$h} = $s;
} 

foreach $header (sort {length($seqs{$a}) <=> length($seqs{$b})} keys %seqs) {
  print $header.$seqs{$header}
}