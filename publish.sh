gutenberg build && \
git add . & \
git commit -m "Save unsafed work" & \
git branch -D master && \
git checkout -b master && \
cp -r public/* . && \
git add . && \
git commit -m "publish" && \
git push origin master -f && \
git checkout source
