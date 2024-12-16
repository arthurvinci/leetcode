//
// Created by arthur on 16/12/24.
//

#ifndef PROBLEM_0142_HPP
#define PROBLEM_0142_HPP

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};


class Solution {
public:
    ListNode *detectCycle(ListNode *head);
};


#endif //PROBLEM_0142_HPP
