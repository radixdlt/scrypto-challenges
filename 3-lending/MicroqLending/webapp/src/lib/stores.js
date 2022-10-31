import { writable } from 'svelte/store';

function createToasts() {
  const timeout = 10000;
  const { subscribe, set } = writable([]);
  let currentUser = null;
  let counter = 0;
  let state = [];
  
  function now(){
    return new Date();
  }
  
  function autoClose(){
    const timestamp = now();
    let newState = state.filter(row => timestamp - row.timestamp < timeout);
    if(newState.length < state.length){
      state = newState;
      set(newState);
    }
  }
  
  function error(message){
    state.push({message, title: "Error", style:"danger", id:counter++, timestamp: now()});
    setTimeout(autoClose, timeout);
    set(state);
  }
  
  function success(message){
    state.push({message, title: "Success", style: "success", id:counter++, timestamp: now()});
    setTimeout(autoClose, timeout);
    set(state);
  }
  
  function remove(id){
    state = state.filter(e => e.id != id);
    set(state);
  }
  
  function clear(){
    state = [];
    set(state);
  }
    
  return {
    subscribe,
    error,
    success,
    remove,
    clear
  };
}

export let toasts = createToasts();