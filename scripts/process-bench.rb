#!/usr/bin/env ruby

# This just processes the CSV data made by Criterion into a table
# that's easier to read in the README

require 'json'

suffix = ARGV[0].strip()

if suffix != "encode" and suffix != "decode"
  STDERR.puts "#{suffix}: argument must be either 'encode' or 'decode'"
  exit 1
end

results = []
cols = ["bench", "size [KB]", "speedup", "flate2 [us]", "libdeflate [us]"]

for dir in Dir.glob("target/criterion/**") do
  group = dir.split("/")[-1].downcase()
  filesize = File.size(File.join("bench_data", group))
  flate2_avg = JSON.parse(File.read(File.join(dir, "flate2_#{suffix}", "new", "estimates.json")))["Mean"]["point_estimate"]
  libdeflate_avg = JSON.parse(File.read(File.join(dir, "libdeflate_#{suffix}", "new", "estimates.json")))["Mean"]["point_estimate"]
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

results = results.sort_by { |results| results["bench"] }

headers = cols.map do |col|
  { "label" => col, "width" => results.map { |result| result[col].size }.append(col.size).max + 2 }
end

puts headers.map { |header| header["label"].ljust(header["width"]) }.join("  ")

for result in results do
  puts headers.map { |header| result[header["label"]].ljust(header["width"]) }.join("  ")
end
