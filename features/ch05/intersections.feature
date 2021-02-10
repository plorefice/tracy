Feature: Intersections

  Scenario: An intersection encapsulates t and object
    Given s ← sphere()
    When i ← intersection(3.5, s)
    Then i.t = 3.5
    And i.object = s
