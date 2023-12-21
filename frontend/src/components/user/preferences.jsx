import usePersistentState from "../../_helpers/UsePersistent";


export default function PreferencesElement() {
    const [prefs,setPrefs] = usePersistentState("preferences",[]);
    
}