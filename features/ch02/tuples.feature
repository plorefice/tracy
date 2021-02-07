Feature: Tuples, Vectors, and Points

  Scenario: Colors are (red, green, blue) tuples
    Given c ← color(-0.5, 0.4, 1.7)
    Then c.red = -0.5
    And c.green = 0.4
    And c.blue = 1.7

  Scenario: Adding colors
    Given c1 ← color(0.9, 0.6, 0.75)
    And c2 ← color(0.7, 0.1, 0.25)
    Then c1 + c2 = color(1.6, 0.7, 1.0)

  Scenario: Subtracting colors
    Given c1 ← color(0.9, 0.6, 0.75)
    And c2 ← color(0.7, 0.1, 0.25)
    Then c1 - c2 = color(0.2, 0.5, 0.5)

  Scenario: Multiplying a color by a scalar
    Given c ← color(0.2, 0.3, 0.4)
    Then c * 2 = color(0.4, 0.6, 0.8)

  Scenario: Multiplying colors
    Given c1 ← color(1, 0.2, 0.4)
    And c2 ← color(0.9, 1, 0.1)
    Then c1 * c2 = color(0.9, 0.2, 0.04)

  Scenario: Reflecting a vector approaching at 45°
    Given v ← vector(1, -1, 0)
    And n ← vector(0, 1, 0)
    When r ← reflect(v, n)
    Then r = vector(1, 1, 0)

  Scenario: Reflecting a vector off a slanted surface
    Given v ← vector(0, -1, 0)
    And n ← vector(√2/2, √2/2, 0)
    When r ← reflect(v, n)
    Then r = vector(1, 0, 0)
