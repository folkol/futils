use std::io::{BufWriter, stdin, stdout, Write};
use std::io::ErrorKind::BrokenPipe;

/**
https://en.wikipedia.org/wiki/Reservoir_sampling
(* S has items to sample, R will contain the result *)
ReservoirSample(S[1..n], R[1..k])
  // fill the reservoir array
  for i := 1 to k
      R[i] := S[i]
  end

  // replace elements with gradually decreasing probability
  for i := k+1 to n
    (* randomInteger(a, b) generates a uniform integer from the inclusive range {a, ..., b} *)
    j := randomInteger(1, i)
    if j <= k
        R[j] := S[i]
    end
  end
end
 */
fn main() {
    let k = 10;
    let mut lines = stdin().lines().map_while(Result::ok).enumerate();
    let mut reservoir: Vec<String> = lines.by_ref().take(k).map(|(_, x)| x).collect();
    for (i, line) in lines {
        let j = fastrand::usize(0..=i);
        if j < reservoir.len() {
            reservoir[j] = line;
        }
    }

    let mut out = BufWriter::new(stdout().lock());
    for item in reservoir {
        match writeln!(out, "{item}") {
            Ok(_) => {}
            Err(e) if e.kind() == BrokenPipe => {}
            Err(_) => panic!("Unexpected error"),
        }
    }
}
