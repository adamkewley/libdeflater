#!/usr/bin/env ruby

# This just processes the CSV data made by Criterion into a table
# that's easier to read in the README

def avg_ns_from_csv(csv)
  measured_vals = csv.lines().drop(1).map { |line| line.split(",")[5].to_f }.to_a()
  (measured_vals.inject(:+) / measured_vals.size)
end

results = []
cols = ["bench", "size [KB]", "flate2 [us]", "libdeflate [us]", "speedup"]

for dir in Dir.glob("target/criterion/**") do
  group = dir.split("/")[-1].downcase()
  filesize = File.size(File.join("bench_data", group))
  flate2_csv = File.read(File.join(dir, "flate2", "new", "raw.csv"))
  flate2_avg = avg_ns_from_csv(flate2_csv)
  libdeflate_csv = File.read(File.join(dir, "libdeflate", "new", "raw.csv"))
  libdeflate_avg = avg_ns_from_csv(libdeflate_csv)
  speedup = (flate2_avg.to_f()/libdeflate_avg.to_f()).round(1).to_s()

  result = {
    "bench" => group,
    "size [KB]" => (filesize / 1000).to_s(),
    "flate2 [us]" => (flate2_avg / 1000).round(0).to_s(),
    "libdeflate [us]" => (libdeflate_avg / 1000).round(0).to_s(),
    "speedup" => speedup,
  }

  results.push(result)
end

headers = cols.map do |col|
  { "label" => col, "width" => results.map { |result| result[col].size }.append(col.size).max + 2 }
end

puts headers.map { |header| header["label"].ljust(header["width"]) }.join("  ")

for result in results do
  puts headers.map { |header| result[header["label"]].ljust(header["width"]) }.join("  ")
end
