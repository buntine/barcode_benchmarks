require 'barby'
require 'barby/barcode/code_128'
require 'barby/outputter/png_outputter'

puts "Start"
5000.times do
barcode = Barby::Code128B.new('BARBY')
barcode.to_png
end
puts "Done"
