require 'barby'
require 'barby/barcode/code_128'
require 'barby/outputter/ascii_outputter'

puts "Start"
100000.times do
barcode = Barby::Code128B.new('BARBY')
barcode.to_ascii
end
puts "Done"
