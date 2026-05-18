Feature: System administration

  Scenario: A system admin can create a group
    Given user "alice" is a system admin
    When user "alice" creates a group
    Then the response status is 201

  Scenario: A non-admin cannot create a group
    Given no preconditions
    When user "bob" creates a group
    Then the response status is 403
