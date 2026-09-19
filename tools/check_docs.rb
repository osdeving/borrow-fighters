#!/usr/bin/env ruby
# Same link/YAML checks as CI, optionally limited to edited files.
require 'yaml'

files = ARGV.empty? ? Dir['*.md', 'docs/**/*.md', '.agents/**/*.md', '.claude/**/*.md',
                         '.github/**/*.{yml,yaml}', '.agents/**/*.yaml', '.claude/**/*.yaml'] : ARGV
bad = []
files.uniq.each do |file|
  unless File.file?(file)
    bad << "missing file: #{file}"
    next
  end
  if file.end_with?('.yml', '.yaml')
    begin
      YAML.load_file(file)
    rescue StandardError => e
      bad << "#{file}: #{e.message}"
    end
  elsif file.end_with?('.md')
    File.read(file).scan(/\[[^\]]+\]\(([^)#]+)(?:#[^)]+)?\)/).flatten.each do |link|
      next if link.match?(%r{^[a-z]+:})
      path = File.expand_path(link, File.dirname(file))
      bad << "#{file}: #{link}" unless File.exist?(path)
    end
  end
end
abort bad.join("\n") unless bad.empty?
puts "Links/YAML: #{files.uniq.length} files OK"
