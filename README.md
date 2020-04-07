# logrs

A small logging library written in rust.  
Can log on 4 level: `INFO`, `DEBUG`, `WARNING`, and `ERROR`.  
Each level can be configured to write to a specific output.  

## Example

``` rust
let mut log = Logger::new().unwrap();

log.config_info(OutputKind::STDOUT);
log.config_debug(OutputKind::STDERR);
log.config_warning(OutputKind::FILE("errlog.txt"));
log.config_error(OutputKind::FILE("errlog.txt"));

log.info("informations.");
log.warning("my warning.");
log.debug("more informations.");
log.error("an error.");
```  
Display of `sdtout`:  
```
[INFO-2020-04-07T15:42:31+0000]: informations.
```  
Display of `sdterr`:  
```
[DEBUG-2020-04-07T15:42:31+0000]: more informations.
```  
Content of `errlog.txt`:  
```
[WARNING-2020-04-07T15:42:31+0000]: my warning.
[ERROR-2020-04-07T15:42:31+0000]: an error.
```