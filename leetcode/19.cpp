// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
   public:
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode* early = head;
        ListNode* late = head;
        while (n--) {
            early = early->next;
        }
        if (early == nullptr) {
            ListNode* next = head->next;
            delete head;
            return next;
        }
        while (early->next != nullptr) {
            early = early->next;
            late = late->next;
        }
        ListNode* next = late->next;
        late->next = next->next;
        delete next;
        return head;
    }
};