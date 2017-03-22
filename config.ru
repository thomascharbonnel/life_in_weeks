require "roda"
require "slim"

BIRTH_DATE = Date.new 1992, 9, 30

class TimeLived
  # Number of weeks lived so far
  def self.weeks
    ((Date.today - BIRTH_DATE) / 7).to_i
  end

  # Number of years lived so far
  def self.years
    (weeks / 52.1429).to_i
  end
end

# Only one webpage, this will be easy.
class App < Roda
  plugin :render, engine: "slim"

  route do |r|
    r.get ["", true] do
      # That's less than 52.1249 weeks / year, but good enough
      @weeks_row, @weeks_col = TimeLived.weeks.divmod(52)
      @decades, @years = TimeLived.years.divmod(10)

      render "life"
    end
  end
end

run App.freeze.app
