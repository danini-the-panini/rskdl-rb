# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/test_*.rb"]
end

require "rb_sys/extensiontask"

task build: :compile

RbSys::ExtensionTask.new("rskdl") do |ext|
  ext.lib_dir = "lib/rskdl"
end

task default: %i[compile test]
