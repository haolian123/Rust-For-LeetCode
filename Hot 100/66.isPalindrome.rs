impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
      if head.is_none()||head.as_ref().unwrap().next.is_none(){
        return true;
      }
      let (mut slow,mut fast) = (head.clone(),head.clone());
      let mut prev:Option<Box<ListNode>> = None;
      while fast.is_some() && fast.as_ref().unwrap().next.is_some(){
        fast = fast.unwrap().next.unwrap().next;
        let next = slow.as_mut().unwrap().next.take();
        slow.as_mut().unwrap().next = prev;
        prev = slow;
        slow = next;
      }
      if fast.is_some(){
        slow = slow.unwrap().next;
      }
      Self::check(prev, slow)
    }
    fn check(l1:Option<Box<ListNode>>,l2:Option<Box<ListNode>>) -> bool{
      let (mut l1,mut l2) = (l1,l2);
      while l1.is_some() && l2.is_some(){
        if l1.as_ref().unwrap().val != l2.as_ref().unwrap().val{
          return false;
        }
        l1 = l1.unwrap().next;
        l2 = l2.unwrap().next;
      }
      l1.is_none() && l2.is_none()
    }
  }
  