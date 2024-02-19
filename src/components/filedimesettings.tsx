//exclude hidden
//restore tabs on open
//include folder in search
//show size of folders
//save history of tabs
//load search list
//showchildfoldercount
//config folder location
//display system info using os api
import React, { useEffect, useState } from "react";
import EachSetting from "./switchsettingseach"
import { invoke } from "@tauri-apps/api/tauri";
import { FolderIcon } from "lucide-react";
import { stateinfo } from "../shared/tstypes";
import Link from "next/link";
import { Button } from "./ui/button";
import { useToast } from "./ui/use-toast";
import { Toaster } from "./ui/toaster";

function reloadsize(togglewhat="size"){
    console.log("loading size js---->1");
      const thensobj={
      windowname:"",
      togglewhat:togglewhat
    };
    invoke(
      "nosize",
      thensobj);
    console.log("loading size js----->2")
  }


export default function FiledimeSettings(){
    // const { theme, setTheme } = useTheme()
    const [datafromstngs,setdfs]=useState<React.JSX.Element>()
    useEffect(()=>{
        invoke("configfolpath",{}).then((e)=>{
            console.log(e)
            let stateinf=JSON.parse(e) as stateinfo;
            setdfs(<>
                <EachSetting name="Exclude hidden files" callback={()=>{reloadsize("excludehidden")}} currentstatus={stateinf.excludehidden}/>
                    <EachSetting name="Restore tabs on open" callback={()=>{reloadsize("sessionsave")}} currentstatus={stateinf.sessionstore}/>
                    <EachSetting name="Include folder names in search" callback={()=>{reloadsize("includefolder")}} currentstatus={stateinf.includefolder}/>
                    <EachSetting name="Compute folder sizes" callback={()=>{reloadsize()}} currentstatus={!stateinf.folsize}/>
                    <EachSetting name="Estimate folder child count" callback={()=>{reloadsize("folcount")}} currentstatus={stateinf.childcount}/>
                    <p className="font-semibold">Config files are stored @ {stateinf.cfpath} ({stateinf.cfpathsize})</p>
                </>)
        })
        
    },[])
    const { toast } = useToast()
    return (
    <>
    <div className="w-full h-full flex flex-col items-center overflow-scroll p-4 gap-2">
    <Toaster />
        <div className="flex flex-row font-semibold items-center gap-2">

    <FolderIcon className="h-6 w-6" />
              <span className="">Filedime</span>
        </div>
        {datafromstngs}
        <div className="font-bold text-center">
            Make the app better, just submit Pull Request after making changes.<br/> Source code available <Link target="_blank" className="text-blue-600" href={"https://github.com/visnkmr/wfmossfrontend"}>here</Link>
        </div>
        <div>
        <Button variant={"outline"} onClick={()=>{
                invoke("checker",{}).then((r)=>{
                    console.log(r);
                    // useEffect(()=>{
                        let fname=async ()=>{

                            const cv = await(await import('@tauri-apps/api/app')).getVersion()
                            
                            if( r!==cv){
                              toast({
                                variant:"destructive",
                                title: "Update available",
                                description: `v${r} is available fordownload`,
                                action: <Button variant={"outline"}><Link target="_blank" href="https://github.com/visnkmr/filedime/releases/latest">Update</Link></Button>,
                              })
                    
                            }
                        }
                        fname();
                    // },[])
                  })
            }}>Check for update</Button>
        </div>
    </div>
    </>
    );
}
