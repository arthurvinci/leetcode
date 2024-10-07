
 #[derive(PartialEq, Eq, Clone, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }

 impl ListNode {
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }

 struct Solution {}

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut answer = ListNode::new(0);
        let mut current = &mut answer;

        while l1.is_some() || l2.is_some() {
            let sum = match (&l1, &l2) {
                (Some(l1), Some(l2)) => l1.val + l2.val + carry,
                (Some(l1), None) => l1.val + carry,
                (None, Some(l2)) => l2.val + carry,
                (None, None) => carry
            };

            carry = sum/10;
            current.next = Some(Box::new(ListNode::new(sum%10)));
            current = current.next.as_mut().unwrap();

            l1 = l1.and_then(|l1| l1.next);
            l2 = l2.and_then(|l2| l2.next);
        }

        if carry !=0{
            current.next = Some(Box::new(ListNode::new(1)))
        }


        answer.next
    }
}

 #[cfg(test)]
 mod test{
     use crate::problem_0002::{ListNode, Solution};

     #[test]
     fn test_1(){
         let l1 = Box::new(ListNode{
             val: 2,
             next: Some(Box::new(ListNode{
                 val: 4,
                 next: Some(Box::new(
                     ListNode{
                         val: 3,
                         next: None
                     }
                 ))
             }))
         });

         let l2 = Box::new(ListNode{
             val: 5,
             next: Some(Box::new(ListNode{
                 val: 6,
                 next: Some(Box::new(
                     ListNode{
                         val: 4,
                         next: None
                     }
                 ))
             }))
         });

         let res = Solution::add_two_numbers(Some(l1), Some(l2));
         let expected_res = Some(
             Box::new(ListNode{
                 val: 7,
                 next: Some(Box::new(ListNode{
                     val: 0,
                     next: Some(Box::new(
                         ListNode{
                             val: 8,
                             next: None
                         }
                     ))
                 }))
             })
         );
         assert_eq!(res, expected_res)
     }

 }