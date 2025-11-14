# !/usr/bin/env ruby

require 'json'
require 'fileutils'

class CardGenerator
  def initialize
    @templates = load_templates
    @output_dir = 'generated_cards'
    FileUtils.mkdir_p(@output_dir)
  end

  def generate_card(name, template_name, overrides = {})
    template = @templates[template_name]
    raise "Template '#{template_name}' not found" unless template

    card_data = template.merge(overrides)

    # Generate the Janet code
    janet_code = generate_janet_code(card_data)

    # Write to file
    filename = "#{@output_dir}/#{name.downcase.gsub(/\s+/, '_')}.janet"
    File.write(filename, janet_code)

    puts "Generated: #{filename}"
  end

  def generate_random_card(name = nil)
    name ||= generate_random_name
    template = @templates.values.sample

    # Add some randomization
    overrides = {
      'cost' => rand(1..5),
      'attack-strength' => rand(1..4),
      'defense' => rand(1..3),
      'movement-points' => rand(0..3)
    }

    generate_card(name, @templates.key(template), overrides)
  end

  def list_templates
    puts 'Available templates:'
    @templates.keys.each { |name| puts "  - #{name}" }
  end

  def generate_batch(count = 5)
    puts "Generating #{count} random cards..."
    count.times { generate_random_card }
  end

  private

  def load_templates
    {
      'basic_unit' => {
        'cost' => 2,
        'movement' => '(std/plus 1)',
        'attack' => '(std/plus 1)',
        'movement-points' => 2,
        'attack-strength' => 2,
        'defense' => 2,
        'abilities' => [],
        'card-image' => 'assets/default.png'
      },
      'ranged_unit' => {
        'cost' => 2,
        'movement' => '(std/plus 1)',
        'attack' => '(array/join (std/plus 1) (std/plus 2))',
        'movement-points' => 2,
        'attack-strength' => 2,
        'defense' => 1,
        'abilities' => [],
        'card-image' => 'assets/default.png'
      },
      'heavy_unit' => {
        'cost' => 3,
        'movement' => '(std/plus 1)',
        'attack' => '(std/plus 1)',
        'movement-points' => 1,
        'attack-strength' => 3,
        'defense' => 3,
        'abilities' => [],
        'card-image' => 'assets/default.png'
      },
      'support_unit' => {
        'cost' => 2,
        'movement' => '(std/plus 1)',
        'attack' => '@[]',
        'movement-points' => 2,
        'attack-strength' => 0,
        'defense' => 2,
        'abilities' => ['heal'],
        'card-image' => 'assets/default.png'
      },
      'structure' => {
        'cost' => 1,
        'movement' => '@[]',
        'attack' => '(std/cross 2)',
        'movement-points' => 0,
        'attack-strength' => 2,
        'defense' => 3,
        'abilities' => [],
        'card-image' => 'assets/default.png'
      }
    }
  end

  def generate_janet_code(data)
    code = []

    # Basic properties
    code << "(def cost #{data['cost']})"
    code << "(def card-image \"#{data['card-image']}\")"
    code << ''

    # Movement and attack
    code << "(def movement #{data['movement']})"
    code << "(def movement-points #{data['movement-points']})"
    code << ''
    code << "(def attack #{data['attack']})"
    code << ''

    # Abilities
    abilities_str = data['abilities'].empty? ? '@[]' : "@#{data['abilities'].inspect}"
    code << "(def abilities #{abilities_str})"
    code << ''

    # Combat stats
    code << "(def attack-strength #{data['attack-strength']})"
    code << "(def defense #{data['defense']})"
    code << ''

    # Event handlers
    events = %w[on-draw on-play on-discard on-ability on-turn-begin on-turn-end]
    events.each do |event|
      code << if data[event] && !data[event].empty?
                "(def #{event} #{format_event_handler(data[event])})"
              else
                "(def #{event} @[])"
              end
    end

    code << ''
    code.join("\n")
  end

  def format_event_handler(handler)
    if handler.is_a?(String)
      handler
    elsif handler.is_a?(Array)
      "@[#{handler.join(' ')}]"
    else
      '@[]'
    end
  end

  def generate_random_name
    prefixes = %w[Ancient Dark Light Fire Ice Storm Shadow Blood Iron Stone]
    suffixes = %w[Warrior Mage Knight Archer Guardian Beast Dragon Spirit Golem]

    "#{prefixes.sample} #{suffixes.sample}"
  end
end

# CLI Interface
if __FILE__ == $0
  generator = CardGenerator.new

  case ARGV[0]
  when 'generate', 'gen'
    if ARGV.length < 3
      puts "Usage: #{$0} generate <name> <template> [cost] [attack] [defense]"
      puts "Example: #{$0} generate 'Fire Mage' ranged_unit 3 2 1"
      generator.list_templates
      exit 1
    end

    name = ARGV[1]
    template = ARGV[2]

    overrides = {}
    overrides['cost'] = ARGV[3].to_i if ARGV[3]
    overrides['attack-strength'] = ARGV[4].to_i if ARGV[4]
    overrides['defense'] = ARGV[5].to_i if ARGV[5]

    generator.generate_card(name, template, overrides)

  when 'random', 'rand'
    count = ARGV[1] ? ARGV[1].to_i : 1
    count.times { generator.generate_random_card }

  when 'batch'
    count = ARGV[1] ? ARGV[1].to_i : 5
    generator.generate_batch(count)

  when 'templates', 'list'
    generator.list_templates

  when 'custom'
    # Interactive mode for custom cards
    puts '=== Custom Card Generator ==='
    print 'Card name: '
    name = gets.chomp

    puts "\nAvailable templates:"
    generator.list_templates
    print "\nChoose template: "
    template = gets.chomp

    print 'Cost (default from template): '
    cost_input = gets.chomp
    print 'Attack strength (default from template): '
    attack_input = gets.chomp
    print 'Defense (default from template): '
    defense_input = gets.chomp

    overrides = {}
    overrides['cost'] = cost_input.to_i unless cost_input.empty?
    overrides['attack-strength'] = attack_input.to_i unless attack_input.empty?
    overrides['defense'] = defense_input.to_i unless defense_input.empty?

    generator.generate_card(name, template, overrides)

  else
    puts 'Card Generator for Janet-based Game'
    puts ''
    puts 'Usage:'
    puts "  #{$0} generate <name> <template> [cost] [attack] [defense]"
    puts "  #{$0} random [count]                    # Generate random cards"
    puts "  #{$0} batch [count]                     # Generate batch of cards"
    puts "  #{$0} custom                            # Interactive card creation"
    puts "  #{$0} templates                         # List available templates"
    puts ''
    puts 'Examples:'
    puts "  #{$0} generate 'Fire Warrior' heavy_unit 4 3 2"
    puts "  #{$0} random 3"
    puts "  #{$0} batch 10"

    generator.list_templates
  end
end
