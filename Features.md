###Features and suggestions to to ad the the code

- [ ] Use and ORM to avoid SQL injection
- [ ] implement rate limiting _important_
- [ ] Integrate FCM notifications _important_
- [ ] Integrate cloudinary for image processing _not very urgent_
- [ ] give each transaction a unique identifier _not urgent but easy_
- [x] Integrate prometheus and grafana for server monitoring
- [x] Authentication and route protection
- [x] Refactor the clone part to make it more readable and cleaner


### monitoring set

We are using prometheus and grafana for monitoring
the docker setup for the server monitory is in the metrics/prometheus.yml file

to run the docker file make sure docker is running and type 

```bash
docker run -d -p 9091:9091 -v "$(pwd)/metrics/prometheus.yml:/etc/prometheus/prometheus.yml" prom/prometheus
```


access the endpoints for 

- prometheus: [prometheus](http://localhost:9090/)

- metrics: [metrics](http://localhost:9091/metrics)


for running grafana using docker without locally installing it use



```bash
docker run -d -p 3001:3000 --name=grafana grafana/grafana-enterprice

- 

<!-- | Task           | Time required | Assigned to   | Current Status | Finished | 
|----------------|---------------|---------------|----------------|-----------|
| Calendar Cache | > 5 hours  |  | in progress | - [x] ok?
| Object Cache   | > 5 hours  |  | in progress | [x] item1<br/>[ ] item2
| Object Cache   | > 5 hours  |  | in progress | <ul><li>- [x] item1</li><li>- [ ] item2</li></ul>
| Object Cache   | > 5 hours  |  | in progress | <ul><li>[x] item1</li><li>[ ] item2</li></ul> -->


<!-- - [x] works
- [x] works too


| Unchecked | Checked |
| --------- | ------- |
| &#9744;   | &#9745; | -->