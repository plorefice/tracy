Feature: Spheres

  Scenario: The normal on a sphere at a point on the x axis
    Given s ← sphere()
    When n ← normal_at(s, point(1, 0, 0))
    Then n = vector(1, 0, 0)
  Scenario: The normal on a sphere at a point on the y axis
    Given s ← sphere()
    When n ← normal_at(s, point(0, 1, 0))
    Then n = vector(0, 1, 0)

  Scenario: The normal on a sphere at a point on the z axis
    Given s ← sphere()
    When n ← normal_at(s, point(0, 0, 1))
    Then n = vector(0, 0, 1)

  Scenario: The normal on a sphere at a nonaxial point
    Given s ← sphere()
    #                             √3/3     √3/3     √3/3
    When n ← normal_at(s, point(0.57735, 0.57735, 0.57735))
    Then n = vector(0.57735, 0.57735, 0.57735)

  Scenario: The normal is a normalized vector
    Given s ← sphere()
    When n ← normal_at(s, point(0.57735, 0.57735, 0.57735))
    Then n = normalize(n)

  Scenario: Computing the normal on a translated sphere
    Given s ← sphere()
    And set_transform(s, translation(0, 1, 0))
    When n ← normal_at(s, point(0, 1.70711, -0.70711))
    Then n = vector(0, 0.70711, -0.70711)

  Scenario: Computing the normal on a transformed sphere
    Given s ← sphere()
    And m ← scaling(1, 0.5, 1) * rotation_z(0.62832)
    And set_transform(s, m)
    When n ← normal_at(s, point(0, 0.70711, -0.70711))
    Then n = vector(0, 0.97014, -0.24254)
