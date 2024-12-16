//
// Created by arthur on 16/12/24.
//

#include "problem_0142.hpp"

ListNode *Solution::detectCycle(ListNode *head) {
    ListNode *slow1 = head;
    ListNode *fast = head;

    while(fast != nullptr && fast->next != nullptr) {
        fast = fast->next->next;
        slow1 = slow1->next;

        if(fast == slow1) {
            break;
        }
    }

    if (!fast || !fast->next) {
        return nullptr;
    }
    else {
        ListNode *slow2 = head;

        while(slow1 != slow2) {
            slow1 = slow1->next;
            slow2 = slow2->next;
        }

        return slow2;
    }
}
