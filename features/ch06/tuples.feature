Feature: Tuples, Vectors, and Points

  Scenario: Reflecting a vector approaching at 45°
    Given v ← vector(1, -1, 0)
    And n ← vector(0, 1, 0)
    When r ← reflect(v, n)
    Then r = vector(1, 1, 0)

  Scenario: Reflecting a vector off a slanted surface
    Given v ← vector(0, -1, 0)
    And n ← vector(0.70711, 0.70711, 0)
    When r ← reflect(v, n)
    Then r = vector(1, 0, 0)
