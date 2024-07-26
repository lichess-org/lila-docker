### What are Gitpod prebuilds?

Gitpod prebuilds is a feature that allows you to prebuild your workspace before you actually need it. This means that when you open your workspace, it will be ready to use immediately, without having to wait for the build process to complete. This can save you a lot of time and make your development process more efficient.

### How lila-docker uses Gitpod prebuilds?

lila-docker uses Gitpod prebuilds to speed up the development process. When gitpod prebuilds are enabled for lila-docker, Gitpod will automatically build the workspace(clone repos, pull docker images, populate db and compile lila) in the background, so that it is ready to use as soon as you open it.

### How to enable Gitpod prebuilds for lila-docker?

To enable Gitpod prebuilds for lila-docker, follow these steps:

1. Fork the lila-docker repository, if you haven't already.
   ![Fork](https://github.com/user-attachments/assets/45ceef96-8586-4db1-adb1-9213c95dbbe5)

2. After forking the repository, go to https://gitpod.io/repositories and add your fork of the lila-docker repository.
   ![Add repository](https://github.com/user-attachments/assets/78233aa9-1feb-4970-a8d3-c23536840c6f)
   ![Add your fork](https://github.com/user-attachments/assets/e654993d-b618-4f9e-a04c-47badee666ef)

3. Once the repository is added, go to the repository settings and then to the "Prebuilds" tab and enable prebuilds for the repository, set commit interval to 0 and choose your preferred machine type.
   ![Enable prebuilds](https://github.com/user-attachments/assets/d2f340a1-0c63-49af-839b-6d4f668d53f5)

4. Now, go to https://gitpod.io/prebuilds and run a prebuild for the repository you just added.
   ![Run prebuild](https://github.com/user-attachments/assets/bf3c4284-23c7-49c5-9329-77b683a8812f)

5. Once the prebuild is complete, you can open the workspace and start using it immediately.

That's it! You have now enabled Gitpod prebuilds for lila-docker and can enjoy a faster and more efficient development process.
To start a new workspace with prebuilds, go to https://gitpod.io/workspaces and open the workspace for your fork of the lila-docker repository or you can use `https://gitpod.io/new/#<link-to-your-fork>`. Eg: https://gitpod.io/new/#https://github.com/your-username/lila-docker/tree/main

### Tips for using Gitpod prebuilds with lila-docker

Here are some tips for using Gitpod prebuilds with lila-docker:

- Try to keep your prebuilds up to date by running them regularly by following step 4 and don't forget to update your fork of the lila-docker repository.
- Don't use prebuilds if they are 4-5 days old, we recommend to run a new prebuild or to start a new workspace without prebuilds from the `github.com/lichess-org/lila-docker` repository instead of your fork.
