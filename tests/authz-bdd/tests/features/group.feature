Feature: Group membership and administration

  Scenario: A group admin can add members
    Given user "alice" is a system admin
    And user "alice" creates a group
    When user "alice" adds user "bob" to the group
    Then the response status is 204

  Scenario: A non-admin cannot add members to a group
    Given user "alice" is a system admin
    And user "alice" creates a group
    When user "carol" adds user "bob" to the group
    Then the response status is 403

  Scenario: A group admin can remove members
    Given user "alice" is a system admin
    And user "alice" creates a group
    And user "alice" adds user "bob" to the group
    When user "alice" removes user "bob" from the group
    Then the response status is 204

  Scenario: A non-admin cannot remove members from a group
    Given user "alice" is a system admin
    And user "alice" creates a group
    And user "alice" adds user "bob" to the group
    When user "carol" removes user "bob" from the group
    Then the response status is 403
