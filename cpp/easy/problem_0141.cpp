//
// Created by arthur on 16/12/24.
//

#include "problem_0141.hpp"

bool Solution::hasCycle(ListNode *head) {
    ListNode *slow = head;
    ListNode *fast = head;

    while(fast != nullptr && fast->next != nullptr) {
        fast = fast->next->next;
        slow = slow->next;

        if(fast == slow) {
            return true;
        }
    }

    return false;
}
