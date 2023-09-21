---
name: Batch fuzzing failure
about: Batch fuzzing run failed and needs investigation
title: ClusterFuzzLite Batch Fuzzing Failure {{ date | date('dddd, MMMM Do yyyy') }}
labels: 'batch-fuzzer'
assignees: 'V0ldek'

---

The action {{ action }} failed the fuzzer run.
Run:  [{{ env.RUN_ID }}](https://github.com/V0ldek/rsonpath/actions/runs/{{ env.RUN_ID }})

This issue is automatically generated and assigned to the maintainer.
Use it to track the progress of the post-mortem.

- [ ] Failure acknowledged.
- [ ] Failure reproduced with the run's test case artifact.
- [ ] Failure minimized using `cargo fuzz tmin`.
- [ ] Follow-up bug issue created.

Please, close this issue after the steps are completed and move any follow-up bugfix
discussions in the resulting bug issue.
