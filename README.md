# KdlRustParser

Use the [kdl-rs](https://github.com/kdl-org/kdl-rs) parser to create [kdl-rb](https://github.com/danini-the-panini/kdl-rb) documents

## Installation
Install the gem and add to the application's Gemfile by executing:

    $ bundle add kdl_rust_parser

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install kdl_rust_parser

## Usage

Just require `kdl_rust_parser` and use KDL normally.

```ruby
require "kdl_rust_parser"

KDL.parse_document(a_string) #=> KDL::Document
```

## Caveats

1. Type conversions are not performed. e.g. `(date)"2020-03-14"` will remain a String. Also, the `type_parsers` parser option is ignored.
2. Very small or large floats will not keep their precision. e.g. `12E-1000` will be `0.0` and `12E+1000` will be `Infinity`.

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake test` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/danini-the-panini/kdl_rust_parser.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
