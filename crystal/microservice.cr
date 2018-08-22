require "http/server"
require "json"
require "kemal"

get "/" do
  response = HTTP::Client.get "https://www.reddit.com/r/politics/hot.json"
  reddit = JSON.parse(response.body)
  children = reddit["data"]["children"].as_a.map { |l| l["data"] }

  JSON.build do |json|
    json.array do
      children.map { |c| json.field "title", c["title"] }
    end
  end
end

Kemal.run
