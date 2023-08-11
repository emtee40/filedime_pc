use std::{path::{PathBuf, Path}, time::{SystemTime, UNIX_EPOCH, Instant, Duration}, fs::{self, File}, sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, Ordering}}, thread, io::{BufReader, BufRead}, collections::HashSet};

use rayon::prelude::*;
use serde::Serialize;
use serde_json::json;
// use rust_search::similarity_sort;
use tauri::{Window, State, Manager};
use walkdir::WalkDir;

use crate::{
  FileItem,
  appstate::*,
  fileitem::*, 
  // partialratio::*, 
  sendtofrontend::*, 
  // loadjs::loadjs
};
use fuzzy_matcher::{*, clangd::fuzzy_match};
#[derive(Clone,Debug,Serialize)]
struct rstr{
  term:String,
  files:HashSet<FileItem>
}
// #[tauri::command]
// async fn  search_trie(path:String,string: String, state: State<'_, AppStateStore>)->Result<(),()>
// {
//   let st=state.st.clone();
//       let mut st=st.lock().unwrap();
//   let string=string.to_lowercase();
//   // println!("fs-----{:?}",st.fuzzy_search(&string,2,5));
//   // println!("fs-----{:?}",st.fuzzy_search(&string,2,5).len());
//   // println!("s--------{:?}",st.search(&string));
//   let tl=parallel_search(st.search(&string,path),string);
//   println!("s--------{:?}",tl.len());
//   if(tl.len()<30){
//     println!("{:?}",tl);
//   }
//   Ok(())
// }
#[tauri::command]
pub async fn  search_try(windowname:String,path:String,string: String,window: Window, state: State<'_, AppStateStore>)->Result<(),()>
//  -> Vec<String> 
 {
  let wname=windowname.clone();
    // populate_try(path, &state);
    if(string.len()<3){
      return Ok(());
    }
 
    let data_clone = Arc::clone(&state.messagetothread);
    let mut write_lock = data_clone.write().unwrap();
      *write_lock=string.clone();
  let now = SystemTime::now();
  let duration = now.duration_since(UNIX_EPOCH).unwrap();
  let startime = duration.as_secs();
  println!("hs----{}",startime);


  

  let map=state.stl.lock().unwrap();
//   let op=state.st.lock().unwrap().search_trie(&string);
//   println!("{}",op.len());
//   if(op.len()<10)
//   {
// // op.split_off(1);
//     println!("{:?}",op)
//   }

// //   let mut results:Vec<String>=map.clone()
// //   .par_iter()
// //   .flat_map(|(_, y)| y.par_iter())
// //   .cloned()
// //   .collect();
// //   // similarity_sort(&mut results, &string);

// //   // Print the sorted results
// //   println!("{}",results.len());
// //   // if(results.len()<10)
// //   {
// // results.split_off(1);
// //     for path in results {
      
// //         println!("{:?}", path);
// //     }
// //   }

 
//   return Ok(());
let update:Vec<u64>=vec![1,2,5,7,10,20,40,65,90,120];

  // if(string.len()>3)
  // {

  //   search_pop(path,string).await;
  // }
  // return Ok(());

  
  let string=string.to_lowercase();
  // search_trie(path,string, state).await;
  // return Ok(());
  
  // thread::spawn({
    // let st=state.searchtry.clone();
    // let vecj=st.lock().unwrap().clone();
    // drop(st);
    // // move||{
    // let strings=parallel_search(vecj,string);
    // let mut gh=Vec::new();
    // let mut ret:HashSet<String>=HashSet::new();




    // let ret:HashSet<String>=
    // map.clone()
    // .par_iter()
    // .filter(
    //   |(i,_)|
    //   i.contains(&string)
    // ).
    // flat_map(
    //   |(_,y)|
    //   {
    //     // if i.contains(&string){
    //       // i.clone()
    //       y.par_iter()
    //     // }
    //   }
    // )
    // .cloned()
    // .inspect(|o|{

    // })
    // .collect();


    let app_handle = window.app_handle();


let m:HashSet<FileItem>=HashSet::new();
// Create a RwLock wrapped in an Arc to share the hashset
let ret = Arc::new(RwLock::new(m));

// Create a clone of the Arc for the other thread
let ret_clone = Arc::clone(&ret);
let ret_clone2 = Arc::clone(&ret);

// Create a boolean flag to indicate whether the search is done or not
let done = Arc::new(RwLock::new(false));
let mut notimes=0;
// Create a clone of the Arc for the other thread
let done_clone = Arc::clone(&done);
starttimer(&windowname,&app_handle);
let string_clone=string.clone();
// Spawn another thread to read and print the hashset periodically
thread::spawn(move || {

  let mut last_print = Instant::now(); // initialize the last print time to the current time
    loop {
      notimes+=1;
      let mut msval=1 as u64;
      let ret = ret_clone.read().unwrap();
      if(ret.len()!=0){
        msval=*update.iter().next().unwrap_or(&120);
        
      }
      if last_print.elapsed() >= Duration::from_millis(msval) { 

        // Read the hashset with a read lock
        if(ret.len()!=0){

          slist(&windowname,&app_handle, &ret, string_clone.clone())
        }
      
     

              
        // println!("{:?}", *ret);
        println!("{:?}", ret.len());
        // Check the flag with a read lock
        let done = done_clone.read().unwrap();
        // If the flag is true, break out of the loop
        if *done ||notimes>50 && ret.len()!=0 {
          // app_handle.emit_to(
          //   "main",
          //   "load-sresults",
          //   serde_json::to_string(&ret.clone()).unwrap(),
          // )
          // .map_err(|e| e.to_string()).unwrap();
          // app_handle.emit_to(
          //   "main",
          //   "load-complete",
          //   "",
          // )
          // .map_err(|e| e.to_string()).unwrap();
        
            break;
        }
        // Drop the lock before sleeping
        
        
        // Drop the lock before reading the hashset
        drop(done);
        // Sleep for some time
        last_print = Instant::now(); // update the last print time to the current time
      }
      drop(ret);
        thread::sleep(std::time::Duration::from_millis(30));
    }
});
set_enum_value(&state.whichthread, wThread::Searching);
let stop_flag_local = Arc::new(AtomicBool::new(true));

let stop_flag_ref = Arc::clone(&stop_flag_local);
thread::spawn(move || {
  thread::sleep(Duration::from_secs(1));
  stop_flag_ref.store(false, Ordering::SeqCst);
  // println!("Stop threads from now on");
  // thread::sleep(Duration::from_secs(3));
  // println!("Start threads from now on");
  // stop_flag_ref.store(true, Ordering::SeqCst);

});

// Populate the hashset using par_iter and inspect
let u:HashSet<String>=map.clone()
    .par_iter()
    .filter(|(_,_)|{
      let read_lock = data_clone.read().unwrap();
      if(*read_lock!=string){
        return false;
      }
      let local_thread_controller=stop_flag_local.clone();
      if(!local_thread_controller.load(Ordering::SeqCst)){
        println!("thread stopped by local controller in searchfiles in searchiter");
        return false;
      }
      println!("convert list of search files to par iter {:?}",get_enum_value(&state.whichthread));
      let mut global_thread_controller= true;
        if let wThread::Searching = get_enum_value(&state.whichthread) 
        { global_thread_controller= true; } 
        else 
        { 
          global_thread_controller= false; 
          local_thread_controller.store(false, Ordering::SeqCst);
          println!("thread stopped by global controller in searchfiles");
          return false;
        }
    return true;
    })
    .filter(|(i, _)| {
      fuzzy_match(&i, &string).unwrap_or(0)>0
      //  i.contains(&string)
    })
    .flat_map(|(_, y)| {
      window.emit("reloadlist",json!({
        "message": "pariter2",
        "status": "running",
    }));
      y.par_iter()
    })
    .cloned()
    .filter(|i|{
      !i.contains("node_modules")
    })
    // .inspect(|o| {
    //   // let path=Path::new(o);
    //   // let fname=path.file_name().unwrap().to_string_lossy().to_string();
    //   //   // Write to the hashset with a write lock
    //   //   let mut ret = ret.write().unwrap();
    //   //   fuzzy_match(&fname, &string);
    //   //   ret.insert(populatefileitem(fname, path, &state));
        
    //   //   // Drop the lock after inserting
    //   //   drop(ret);
    // })
    .collect();
  println!("{}",u.len());
  
  
  // if(u.len()<2000)
  {
    let mut v: Vec<String> = u.into_par_iter().collect(); // Collect into a vector
    v
    .par_sort_by_key(|ei| { // Sort by key
      let path = Path::new(&ei); // Get the path
      let fname = path.file_name().unwrap().to_string_lossy().to_string(); // Get the file name
      let score = fuzzy_match(&fname, &string).unwrap(); // Get the score
      // println!("{}---{}", fname, score); // Print the file name and score
      score // Return the score as the key
    });
    v.reverse();

    // v.split_off(100);
    // for (c,ei) in 
    v
    .par_iter()
    .enumerate()
    // .filter(|(_)|{
    //   if let wThread::Searching = get_enum_value(&state.whichthread) { return true; } else { return false; }
    // })
    .filter(|(_,_)|{
      let read_lock = data_clone.read().unwrap();
      if(*read_lock!=string){
        return false;
      }
      println!("convert list of search files to par iter {:?}",get_enum_value(&state.whichthread));

      let local_thread_controller=stop_flag_local.clone();
      if(!local_thread_controller.load(Ordering::SeqCst)){
        eprintln!("thread stopped by local controller in searchfiles sendresp");
        return false;
      }
      println!("loadlist after search complete {:?}",get_enum_value(&state.whichthread));

      let mut global_thread_controller= true;
        if let wThread::Searching = get_enum_value(&state.whichthread) 
        { global_thread_controller= true; } 
        else 
        { global_thread_controller= false; }
      if !global_thread_controller {
        local_thread_controller.store(false, Ordering::SeqCst);
        eprintln!("thread stopped by global controller in searchfiles");
        return false;
    }
    return true;
    })
    .try_for_each(|(c,ei)|{
      window.emit("reloadlist",json!({
        "message": "pariter3",
        "status": "running",
    }));
      // if c>2000{
      //   return None;
      // }
      // let path=Path::new(&ei);
      //   let fname=path.file_name().unwrap().to_string_lossy().to_string();
      //   let score=fuzzy_match(&fname, &string).unwrap();
      let path=Path::new(&ei);
      let fname=path.file_name().unwrap().to_string_lossy().to_string();
        // Write to the hashset with a write lock
        let mut ret = ret.write().unwrap();
        fuzzy_match(&fname, &string);
        ret.insert(populatefileitem(fname, path,&window, &state));
        
        // Drop the lock after inserting
        drop(ret);
      println!("{}",ei);
      Some(())
    });
    let wtr=ret_clone2.read().unwrap().clone();
    slist(&wname,&window.app_handle(), &wtr, string.clone())
  }
  // Set the flag to true with a write lock
let mut done = done.write().unwrap();
*done = true;
stoptimer(&wname,&window.app_handle());
    // for (i,_) in map.clone(){
    //   // gh.push(i);
    //   if i.contains(&string){
    //     gh.push(i.clone());
    //   }
    // }
    // let o=map.clone();
    // let ret:HashSet<String>=gh.par_iter().flat_map(|u|{
    //   let y=o.get(u).unwrap();
    //   // let f:Vec<String>=
    //   y.par_iter()
    //   // .map(|t|{
    //     // t.clone()
    //   // }).collect();
    //   // f
    // }).cloned().collect();

    // for i in gh{
    //   let y=o.get(&i.clone()).unwrap();
    //   for j in y{
    //     ret.insert(j.clone());
    //   }
    // }
    // let ret = ret.read().unwrap();
    // println!("{:?}",ret.len());
    // if(ret.len()<20){
    //   println!("{:?}",ret);
    // }
    // drop(ret);
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap();
    let endtime = duration.as_secs();
    println!("endtime----{}",endtime-startime);

  // }
//  });
 Ok(())
  // strings
  
  // println!("{:?}",options);
  // options
}
// Define a function that takes a vector of strings and a string as parameters
// fn parallel_search(k: HashSet<String>, h: String) -> Vec<String> {
//   // while true{

//   // };
//   // Create a parallel iterator over the vector k
//   k.par_iter()
//       // Filter out the elements that do not contain h
//       .filter(|s| 
//         partial_ratio(s, &h)>80
//         // s.contains(&h)
//       )
//       // .filter(|s| s.contains(&h))
//       // Collect the filtered elements into a new vector
//       .cloned()
//       .collect()
// }
// pub struct Searchresults{
//   name:String,
//   path:String,
//   is_dir: bool,
//   size:String,
//   rawfs:u64,
//   lmdate:String,
//   timestamp:i64,
//   foldercon:i32,
//   ftype:String
// }

pub async fn search_pop(path: String,string:String){
  let string=string.to_lowercase();
  // populate_trie(oid, path, ff, window, state).await;
  // return ;
  
  let now = SystemTime::now();
  let duration = now.duration_since(UNIX_EPOCH).unwrap();
  let startime = duration.as_secs();
  println!("hs----{}",startime);
  // thread::spawn(
    // {
      // let st=state.searchtry.clone();
    
    // move||{
        let walker2 = WalkDir::new(&path)
        // .contents_first(true)
          .min_depth(1) // skip the root directory
          // .max_depth(1) // only look at the immediate subdirectories
          .into_iter()
          
          .filter_entry(
            |e| 
            !e.path_is_symlink() 
            // &&
            // !e
            // .file_name()
            // .to_str()
            // .map(|s| s.starts_with("."))
            // .unwrap_or(false)
            &&
            !is_hidden(e)
            // &&
            // e.file_name()
            // .to_string_lossy()
            // .to_string().to_lowercase()
            // .contains(&string)
            // &&
            // e.file_type().is_file()
              // e.file_type().is_dir()
          );

        let mut count=RwLock::new(0);
        
        
        let par_walker2 = walker2.par_bridge(); // ignore errors
        
        let k:HashSet<String>=
        par_walker2
        // .enumerate()
        .into_par_iter()
          
        .filter_map(
          |i|
          {
            
          match(i){
            Ok(i) => {
              if((i.file_name()
              .to_string_lossy()
              .to_string().to_lowercase()
              .contains(&string))){
                Some(i.path().to_string_lossy().to_string())
              }
              else{
                None
              }
            },
            Err(_) => None,
          }
        }
        )
        .map(
          |e|
          {
          //   window.emit("reloadlist",json!({
          //     "message": "pariter8",
          //     "status": "running",
          // }));
           e
          })
          .collect();

        println!("{}",k.len());
        if(k.len()<20){
          println!("{:?}",k);
        }

        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).unwrap();
        let endtime = duration.as_secs();
        println!("endtime----{}",endtime-startime);
        
        // .collect(); // collect into a vec
        // let mut st=st.lock().unwrap();
        // *st=(k.clone());
        // println!("-------c ----{}",count.read().unwrap());
        // drop(st);
        
        // for i in k{
        //   let name=PathBuf::from(&i).file_name().unwrap().to_string_lossy().to_string().to_lowercase();
        //   map.entry(name).or_insert(Vec::new()).push(i);

        // }
        // let map: HashMap<String, Vec<String>> = par_walker2
        // .into_par_iter()
        // .filter_map(Result::ok) // ignore errors
        // .map(|e| e.path().to_string_lossy().into_owned()) // get path as string
        // .with_key(|path| {
        //     PathBuf::from(path) // convert to PathBuf
        //         .file_name() // get file name
        //         .unwrap() // unwrap the Option
        //         .to_string_lossy() // convert to string
        //         .to_string() // convert to owned string
        //         .to_lowercase() // convert to lowercase
        // }) // use custom key function
        // .into_par_iter() // convert to parallel iterator
        // .from_par_iter(); // create hashmap from parallel iterator
      // }
    // }
  // );
  
}

