#!/usr/bin/env ruby
require 'open3'
TEST_DIR  = "./test"
BINARY = "./target/release/loxidation"

puts "Building crate"
# Build release to suppress debug messages
system("cargo build --release")
puts "Done building"

test_paths = Dir.entries(TEST_DIR).map {|e| File.join(TEST_DIR, e)}

test_paths = test_paths.reject {|e| e == File.join(TEST_DIR, "..") || e == File.join(TEST_DIR, ".")}

new_paths = []
test_paths = test_paths.select {|path|
    if File.directory? path then
        entries = Dir.glob(File.join path, "*.lox")
        new_paths << entries
        next false
    end
    next true
}

for paths in new_paths
    test_paths.concat paths
end

tests = {}

for test_path in test_paths
   rest, filename = File.split test_path
   rest, category = File.split rest
   if ["test","."].include? category
        category = "standard"
    end
    tests[category] = {} if !tests.has_key? category
    tests[category][filename] = {
        path: test_path,
        passed: false
    }
end

DISABLED_CATEGORIES = ["scanning"]

def compare_output file, output
    output = "" if !output
    src = File.read file
    expect = ""
    line_n = 0
    for line in src.lines
        line_n += 1
        if line.include? "//"
            comment = line.split("//").last.strip
            if comment.start_with? "expect: "
                expect << comment["expect: ".length..] + "\n"
            end
        end
    end
    return expect == output
end

def compare_errors file, output
    output = "" if !output
    src = File.read file
    expect = ""
    line_n = 0
    for line in src.lines
        line_n += 1
        if line.include? "//"
            comment = line.split("//").last.strip
            if comment.start_with? "error: "
                expect << comment["error: ".length..] + "\n"
            end
        end
    end
    new_out = ""
    for line in output.lines
        # Remove front of error
        # https://regexr.com/78p3c
        new_out << (line.gsub!(/^(((Line|Error at line) \d+( at ('.+'|EOF))?)|Error): /, '') || line)
    end
    return expect == new_out
end

for category in tests
    category_name = category[0]
    category_tests = category[1]
    if DISABLED_CATEGORIES.include? category_name
        puts "Skipping #{category_name}"
        next
    end
    puts "Testing #{category_name}"
    for test_ in category_tests
        name = test_[0]
        test_ = test_[1]
        print "Running test #{name}: "
        stdin, stdout, stderr, wait_thr = Open3.popen3 BINARY,  test_[:path]
        err = stderr.gets(nil)
        out = stdout.gets(nil)
        stdin.close
        stdout.close
        stderr.close
        #exit_status = wait_thr.value
        suc = compare_output(test_[:path], out) && compare_errors(test_[:path], err);
        test_[:passed] = suc
        puts '✔' if suc
        puts '⨯' if !suc
    end
    puts
end

puts "\n==Results=="
for category in tests
    category_name = category[0]
    category_tests = category[1]
    next if DISABLED_CATEGORIES.include? category_name
    passed = category_tests.count {|t| t[1][:passed]}
    mark = passed == category_tests.length ? '✔' : '⨯'
    puts "#{category_name}: #{passed}/#{category_tests.length} #{mark}"
end
